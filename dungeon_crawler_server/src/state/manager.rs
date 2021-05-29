use std::{collections::HashMap, time::Duration};

use crate::state::{
    ai::ai_package_collections::{IDLE, MELEE_COMBAT},
    types::ResponseType,
};
use crossbeam::channel::{Receiver, Sender};
use dungeon_generator::inst::Dungeon;
use rand::prelude::*;
use simple_serializer::Serialize;

use super::{
    actor::{Actor, ActorId, Status},
    ai::ai_package_manager::IndependentManager,
    monsters::{Monster, MonsterInstance},
    players::Player,
    traits::Identified,
    transforms::transform::Transform,
};
use super::{
    snapshot::StateSnapshot,
    stats::{Attributes, Stats},
    traits::AI,
    transforms::{transform::Direction, vec2::Vec2, world_stage::WorldStage},
    types::RequestType,
};

///
/// Template definitions for Monsters
///
static MONSTERS: [Monster; 1] = [Monster {
    stats: Stats {
        cur_health: 20,
        max_health: 20,
        cur_stamina: 20,
        max_stamina: 20,
        cur_magicka: 0,
        max_magicka: 0,
    },
    attrs: Attributes {
        might: 2,
        fines: 5,
        intel: 1,
    },
    id: 0,
    name: "Goblin",
    spawn_chance: 10,
    sight_range: 3,
}];

///
/// Controls all server state, and holds
/// a `Sender` and `Receiver`, which can be
/// cloned to communicate with the state
///
pub struct StateManager {
    s_to_state: Sender<RequestType>,
    r_at_event: Receiver<ResponseType>,
}

impl StateManager {
    ///
    /// Create a new `StateHandler` with the supplied `dungeon`,
    /// starting a new state event loop
    ///
    pub fn new(dungeon: Dungeon) -> Self {
        let (s_to_state, r_at_event) = state_loop(dungeon);

        Self {
            s_to_state,
            r_at_event,
        }
    }
    ///
    /// Returns the `Sender` and `Receiver` used to
    /// communicate with the current server state.
    ///
    pub fn get_sender_receiver(&self) -> (Sender<RequestType>, Receiver<ResponseType>) {
        (self.s_to_state.clone(), self.r_at_event.clone())
    }
}

/// The state loop, ran on a separate thread.
/// Receives updates from the `EventManager` and adjusts states
/// accordingly. Runs game logic.
fn state_loop(dungeon: Dungeon) -> (Sender<RequestType>, Receiver<ResponseType>) {
    // Create the mpsc channels connecting the EventHandler to the StateHandler,
    // and vice-versa
    let (s_to_state, r_at_state) = crossbeam::channel::unbounded();
    let (s_to_event, r_at_event) = crossbeam::channel::unbounded();

    std::thread::spawn(move || {
        // The collection of monsters, keyed by position
        let mut monsters = HashMap::<Vec2, MonsterInstance>::new();
        // The collection of players, keyed by id
        let mut players = HashMap::<u32, Player>::new();
        // The collection of AI Managers
        let mut ai_managers = HashMap::<u32, IndependentManager<dyn AI>>::new();

        // Create the new WorldStage, using the supplied dungeon
        let mut world_stage = WorldStage::new(
            dungeon
                .paths_ref()
                .iter()
                .cloned()
                .map(|s| Vec2(s.0, s.1))
                .collect(),
            Vec2::from_tuple(dungeon.entrance),
            Vec2::from_tuple(dungeon.exit),
            s_to_event.clone(),
        );

        // Begin the stateloop, which will not cease until the program ends
        loop {
            // If there are any requests sent to the StateManager
            if let Ok(request) = r_at_state.try_recv() {
                match request {
                    // If a new player has been added, insert them into the
                    // WorldState and send a StateSnapshot to the EventManager,
                    // to be forwarded to the player
                    RequestType::NewPlayer(addr, id, name) => {
                        players.insert(id, Player::new(id, name.clone()));
                        world_stage.add(
                            id,
                            Actor::new(
                                id,
                                Stats::new(10, 10, 10),
                                Attributes::new(5, 5, 5),
                                Transform::with_values(
                                    Vec2::from_tuple(dungeon.entrance),
                                    Direction::Left,
                                ),
                                ActorId::Player,
                            ),
                        );
                        s_to_event
                            .send(ResponseType::StateSnapshot(StateSnapshot {
                                addr_for: addr,
                                new_player: (id, name, world_stage.pos(id).unwrap()),
                                other_players: players
                                    .values()
                                    .filter(|p| p.id != id)
                                    .cloned()
                                    .map(|p| {
                                        (
                                            p.id,
                                            p.name.clone(),
                                            world_stage.pos(p.id).unwrap(),
                                            world_stage.actor(p.id).unwrap().status.serialize(),
                                        )
                                    })
                                    .collect(),
                                monsters: monsters
                                    .values()
                                    .map(|m| {
                                        (m.template.id, m.id(), world_stage.pos(m.id()).unwrap())
                                    })
                                    .collect(),
                                dungeon: dungeon.clone(),
                                all_player_ts: world_stage.clone_transforms(),
                            }))
                            .unwrap();
                    }
                    // If a Player has been dropped, remove them from the
                    // WorldStage and players collection
                    RequestType::DropPlayer(id) => {
                        world_stage.remove(id);
                        players.remove(&id);
                    }
                    // If a Player has move, update their
                    // Transform in the WorldStage
                    RequestType::PlayerMoved(id, new_t) => {
                        world_stage.update_pl_tr(id, new_t);
                    }
                    // If its been requested to spawn a new monster
                    // generate a monster and send its information back
                    RequestType::SpawnMonster(id) => {
                        let monster = spawn_monster(id, &mut world_stage);
                        s_to_event
                            .send(ResponseType::NewMonster(
                                monster.template.id,
                                monster.instance_id,
                                world_stage.pos(monster.id()).unwrap(),
                                world_stage.dir(monster.id()).unwrap(),
                            ))
                            .unwrap();
                        // Insert the monster's AI package into ai_managers
                        ai_managers.insert(
                            monster.instance_id,
                            IndependentManager::new(vec![&IDLE, &MELEE_COMBAT]),
                        );
                        monsters.insert(world_stage.pos(monster.id()).unwrap(), monster);
                    }
                    RequestType::AttemptHit(attk_id, defd_id) => world_stage.try_attk(attk_id, defd_id),
                    // If the program is ending, break from the loop
                    RequestType::Abort => break,
                }
            }

            // If there are no players currently connected
            // do not update enemy AI
            if players.is_empty() {
                continue;
            }
            // Run each monster's AI
            for monster in monsters.values_mut() {
                let index = monster.instance_id;
                if world_stage.actor(index).unwrap().status != Status::Dead {
                    ai_managers
                        .get_mut(&index)
                        .unwrap()
                        .run(&mut world_stage, monster, &s_to_event);
                }
            }
            // Check if the dungeon is complete. If so,
            // update the EventManager to inform the connected clients
            if is_dungeon_complete(&mut world_stage, &players) {
                s_to_event.send(ResponseType::DungeonComplete).unwrap();
                for pl in players.values() {
                    world_stage.actor(pl.id).unwrap().status = Status::Active;
                }
            }

            // Sleep for 10 milliseconds - generally the logic does not
            // need to be updated any faster, and this will conserve
            // processing power
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    (s_to_state, r_at_event)
}

/// Checks if the dungeon is complete by determining if all `players`
/// are either `Escaped` or `Dead`
fn is_dungeon_complete(world_stage: &mut WorldStage, players: &HashMap<u32, Player>) -> bool {
    !players.is_empty()
        && players
            .values()
            .all(|pl| world_stage.actor(pl.id()).unwrap().status != Status::Active)
}

/// Generates a single monster at random with the given `id`
fn spawn_monster(id: u32, world_stage: &mut WorldStage) -> MonsterInstance {
    // Find the sum of all monster's spawn chances
    let rand_count: u32 = MONSTERS.iter().map(|m| m.spawn_chance).sum();

    // Choose a value in the range of rand_count.
    let mut choice = ((thread_rng().next_u32() % rand_count) + 1) as i32;
    let mut index = 0;

    // Choose the monster based on the value of choice. Subtract each monster's
    // spawn_chance from choice. When choice hits 0, that particular monster is chosen
    for monster in MONSTERS.iter() {
        choice -= monster.spawn_chance as i32;
        if choice <= 0 {
            break;
        }
        index += 1;
    }

    // Add the MonsterIntance to the WorldStage
    let open_spot = world_stage.open_spot();
    world_stage
        .add(
            id,
            Actor::new(
                id,
                MONSTERS[index].stats,
                MONSTERS[index].attrs,
                Transform::with_values(open_spot, Direction::Right),
                ActorId::Monster,
            ),
        )
        .unwrap();

    MonsterInstance::new(&MONSTERS[index], id)
}

impl Drop for StateManager {
    fn drop(&mut self) {
        // Drop the state thread when the application quits
        self.s_to_state.send(RequestType::Abort).unwrap();
    }
}
