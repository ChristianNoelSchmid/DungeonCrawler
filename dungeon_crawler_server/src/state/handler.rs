use std::{collections::{HashMap, HashSet}, time::{Duration, Instant}};

use crate::{
    astar::find_shortest_path,
    state::{
        monsters::{Monster, MonsterInstance},
        players::Player,
        types::ResponseType,
    },
};
use crossbeam::channel::{Receiver, Sender};
use dungeon_generator::inst::Dungeon;
use rand::prelude::*;
use simple_serializer::Serialize;

use super::{
    snapshot::StateSnapshot,
    transform::{Direction, Transform},
    types::RequestType,
};

const UPDATE_INCREMENT_MILLIS: u128 = 125;

///
/// Template definitions for Monsters
///
const MONSTERS: [Monster; 2] = [
    Monster {
        template_id: 0,
        name: "Goblin",
        spawn_chance: 10,
        range: 1,
        damage: 2,
    },
    Monster {
        template_id: 1,
        name: "Ghost",
        spawn_chance: 3,
        range: 5,
        damage: 2,
    },
];

///
/// Controls all server state, and holds
/// a `Sender` and `Receiver`, which can be
/// cloned to communicate with the state
///
pub struct StateHandler {
    s_to_state: Sender<RequestType>,
    r_from_state: Receiver<ResponseType>,
}

impl StateHandler {
    ///
    /// Create a new `StateHandler` with the supplied `dungeon`,
    /// starting a new state event loop
    ///
    pub fn new(dungeon: Dungeon) -> Self {
        let (s_to_state, r_from_state) = state_loop(dungeon);

        Self {
            s_to_state,
            r_from_state,
        }
    }
    ///
    /// Returns the `Sender` and `Receiver` used to
    /// communicate with the current server state.
    ///
    pub fn get_sender_receiver(&self) -> (Sender<RequestType>, Receiver<ResponseType>) {
        (self.s_to_state.clone(), self.r_from_state.clone())
    }
}

fn state_loop<'a>(dungeon: Dungeon) -> (Sender<RequestType>, Receiver<ResponseType>) {
    let (s_to_state, r_at_state) = crossbeam::channel::unbounded();
    let (s_from_state, r_from_state) = crossbeam::channel::unbounded();

    let mut monsters = HashMap::<(i32, i32), MonsterInstance>::new();
    let mut players = HashMap::<u32, Player>::new();
    let mut filled_spots = HashSet::new();

    filled_spots.insert(dungeon.entrance);
    filled_spots.insert(dungeon.exit);

    for x in 0..dungeon.width() as i32 {
        for y in 0..dungeon.height() as i32 {
            filled_spots.insert((x, y));
        }
    }
    for path in dungeon.paths() {
        filled_spots.remove(path);
    }

    let mut update_instant = Instant::now();
    std::thread::spawn(move || loop {
        if let Ok(request) = r_at_state.try_recv() {
            match request {
                RequestType::NewPlayer(addr, id) => {
                    players.insert(
                        id,
                        Player {
                            id,
                            name: "".to_string(),
                            transform: Transform::with_values(dungeon.entrance, Direction::Left),
                        },
                    );
                    s_from_state
                        .send(ResponseType::StateSnapshot(StateSnapshot {
                            player_id: id,
                            addr_for: addr,
                            players: players.values().cloned().collect(),
                            monsters: monsters.values().cloned().collect(),
                            paths: dungeon.serialize(),
                            entrance: dungeon.entrance,
                            exit: dungeon.exit,
                        }))
                        .unwrap();
                }
                RequestType::PlayerMoved(id, transform) => {
                    if let Some(player) = players.get_mut(&id) {
                        player.transform.from_other(&mut filled_spots, transform);
                        for monster in monsters.values_mut() {
                            monster.path = find_shortest_path(
                                dungeon.paths_ref(),
                                &filled_spots,
                                monster.transform.pos(),
                                transform.pos(),
                            );
                        }
                    }
                }
                RequestType::SpawnMonster(id) => {
                    let monster = spawn_monster(id, &dungeon, &mut monsters, &mut filled_spots);
                    s_from_state
                        .send(ResponseType::NewMonster(monster))
                        .unwrap();
                }
                RequestType::AStar(position) => {
                    for monster in monsters.values_mut() {
                        monster.path = find_shortest_path(
                            dungeon.paths_ref(),
                            &filled_spots,
                            monster.transform.pos(),
                            position,
                        )
                    }
                }
            }
        }
        if (Instant::now() - update_instant).as_millis() > UPDATE_INCREMENT_MILLIS {
            for monster in monsters.values_mut() {
                if let Some(next) = monster.path.pop() {
                    if !monster.transform.move_pos(&mut filled_spots, next) && monster.path.len() > 0 {
                        monster.path = find_shortest_path(
                            dungeon.paths_ref(),
                            &filled_spots,
                            monster.transform.pos(),
                            monster.path[0],
                        );
                    } else {
                        monster.transform.move_pos(&mut filled_spots, next);
                    }
                    s_from_state
                        .send(ResponseType::MonsterMoved(
                            monster.instance_id,
                            monster.transform,
                        ))
                        .unwrap();
                }
            }
            update_instant = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(100));
    });

    (s_to_state, r_from_state)
}

fn spawn_monster(
    id: u32,
    dungeon: &Dungeon,
    monsters: &mut HashMap<(i32, i32), MonsterInstance>,
    filled_spots: &mut HashSet<(i32, i32)>,
) -> MonsterInstance {
    let rand_count: u32 = MONSTERS.iter().map(|m| m.spawn_chance).sum();
    let mut choice = ((thread_rng().next_u32() % rand_count) + 1) as i32;
    let mut index = 0;

    for monster in MONSTERS.iter() {
        choice -= monster.spawn_chance as i32;
        if choice <= 0 {
            break;
        } else {
            index += 1;
        }
    }

    let open_spot = open_spot(&dungeon, &filled_spots);
    let instance = MonsterInstance {
        template: &MONSTERS[index],
        instance_id: id,
        transform: Transform::with_values(open_spot, Direction::Right),
        path: vec![open_spot],
    };
    monsters.insert(open_spot, instance.clone());
    filled_spots.insert(open_spot);

    instance
}

///
/// Finds a currently open spot on the map,
/// retrieving the current `dungeon`, and checking
/// current `monsters` and `players` positions,
/// filtering them out
///
fn open_spot<'a>(dungeon: &Dungeon, filled_spots: &'a HashSet<(i32, i32)>) -> (i32, i32) {
    *dungeon
        .paths()
        .filter(|path| !filled_spots.contains(path))
        .choose(&mut thread_rng())
        .unwrap()
}

///
/// Finds a currently open spot on the map,
/// retrieving the current `dungeon`, and checking
/// current `monsters` and `players` positions,
/// filtering them out
///
fn open_spot_by<'a>(
    dungeon: &'a Dungeon,
    filled_spots: &'a HashSet<(i32, i32)>,
    spot: (i32, i32),
) -> Option<&'a (i32, i32)> {
    dungeon
        .paths()
        .filter(|path| Transform::distance(**path, spot) <= 1.0)
        .filter(|path| !filled_spots.contains(path))
        .choose(&mut thread_rng())
}
