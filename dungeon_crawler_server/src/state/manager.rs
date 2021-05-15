use std::{collections::HashMap, time::Duration};

use crate::state::{
    ai::ai_package_collections::{IDLE, MELEE_COMBAT},
    types::ResponseType,
};
use crossbeam::channel::{Receiver, Sender};
use dungeon_generator::inst::Dungeon;
use rand::prelude::*;

use super::{
    actor::{Actor, ActorId},
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
static MONSTERS: [Monster; 1] = [
    Monster {
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
        sight_range: 6,
    },
    /*Monster {
        stats: Stats {
            cur_health: 10,
            max_health: 10,
            cur_stamina: 5,
            max_stamina: 5,
            cur_magicka: 10,
            max_magicka: 10,
        },
        attrs: Attributes {
            might: 1,
            fines: 3,
            intel: 5,
        },
        template_id: 1,
        name: "Ghost",
        spawn_chance: 3,
    },*/
];

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

fn state_loop(dungeon: Dungeon) -> (Sender<RequestType>, Receiver<ResponseType>) {
    let (s_to_state, r_at_state) = crossbeam::channel::unbounded();
    let (s_to_event, r_at_event) = crossbeam::channel::unbounded();

    std::thread::spawn(move || -> ! {
        let mut monsters = HashMap::<Vec2, MonsterInstance>::new();
        let mut players = HashMap::<u32, Player>::new();
        let mut world_stage = WorldStage::new(
            dungeon
                .paths_ref()
                .iter()
                .cloned()
                .map(|s| Vec2(s.0, s.1))
                .collect(),
            s_to_event.clone(),
        );
        let mut ai_managers = HashMap::<u32, IndependentManager<dyn AI>>::new();

        loop {
            // RequestType Reception
            if let Ok(request) = r_at_state.try_recv() {
                match request {
                    RequestType::NewPlayer(addr, id) => {
                        players.insert(id, Player::new(id, "".to_string()));
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
                                new_player: (id, "".to_string(), world_stage.pos(id).unwrap()),
                                other_players: players
                                    .values()
                                    .filter(|p| p.id != id)
                                    .cloned()
                                    .map(|p| (p.id, p.name.clone(), world_stage.pos(p.id).unwrap()))
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
                    RequestType::DropPlayer(id) => {
                        players.remove(&id);
                    }
                    RequestType::PlayerMoved(id, new_t) => {
                        world_stage.update_transform(id, new_t);
                    }
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
                        ai_managers.insert(
                            monster.instance_id,
                            IndependentManager::new(vec![&IDLE, &MELEE_COMBAT]),
                        );
                        monsters.insert(world_stage.pos(monster.id()).unwrap(), monster);
                    }
                    _ => {}
                }
            }
            for monster in monsters.values_mut() {
                let index = monster.instance_id;
                ai_managers
                    .get_mut(&index)
                    .unwrap()
                    .run(&mut world_stage, monster, &s_to_event);
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    });

    (s_to_state, r_at_event)
}

fn spawn_monster(id: u32, transformer: &mut WorldStage) -> MonsterInstance {
    let rand_count: u32 = MONSTERS.iter().map(|m| m.spawn_chance).sum();
    let mut choice = ((thread_rng().next_u32() % rand_count) + 1) as i32;
    let mut index = 0;

    for monster in MONSTERS.iter() {
        choice -= monster.spawn_chance as i32;
        if choice <= 0 {
            break;
        }
        index += 1;
    }

    let open_spot = transformer.open_spot();
    transformer
        .add(
            id,
            Actor::new(
                id,
                MONSTERS[index].stats.clone(),
                MONSTERS[index].attrs,
                Transform::with_values(open_spot, Direction::Right),
                ActorId::Monster,
            ),
        )
        .unwrap();

    MonsterInstance::new(&MONSTERS[index], id)
}
