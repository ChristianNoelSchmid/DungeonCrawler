use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::state::{
    monsters::{Monster, MonsterInstance},
    players::Player,
    types::ResponseType,
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

const UPDATE_INCREMENT_MILLIS: u128 = 1000;

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

    let mut monsters = HashMap::<(u32, u32), MonsterInstance>::new();
    let mut players = HashMap::<u32, Player>::new();
    let mut filled_spots = HashSet::new();

    filled_spots.insert(dungeon.entrance);
    filled_spots.insert(dungeon.exit);

    for x in 0..dungeon.width() {
        for y in 0..dungeon.height() {
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
                        filled_spots.remove(&player.transform.position);
                        (*player).transform = transform;
                        filled_spots.insert(transform.position);
                    }
                }
                RequestType::SpawnMonster(id) => {
                    let monster = spawn_monster(id, &dungeon, &mut monsters, &mut filled_spots);
                    s_from_state
                        .send(ResponseType::NewMonster(monster))
                        .unwrap();
                }
            }
        }
        if (Instant::now() - update_instant).as_millis() > UPDATE_INCREMENT_MILLIS {
            for monster in monsters.values_mut() {
                let open_spot = open_spot_by(&dungeon, &filled_spots, monster.transform.position);
                let mut switch_spot = None;
                if let Some(spot) = open_spot {
                    switch_spot = Some((monster.transform.position, *spot));
                    if spot.0 > monster.transform.position.0 {
                        (*monster).transform.direction = Direction::Right;
                    } else if spot.0 < monster.transform.position.0 {
                        (*monster).transform.direction = Direction::Left;
                    }
                    (*monster).transform.position = *spot;

                    s_from_state
                        .send(ResponseType::MonsterMoved(*monster))
                        .unwrap();
                }
                if let Some(spots) = switch_spot {
                    filled_spots.remove(&spots.0);
                    filled_spots.insert(spots.1);
                }
            }
            update_instant = Instant::now();
        } else {
            std::thread::yield_now();
        }
    });

    (s_to_state, r_from_state)
}

fn spawn_monster(
    id: u32,
    dungeon: &Dungeon,
    monsters: &mut HashMap<(u32, u32), MonsterInstance>,
    filled_spots: &mut HashSet<(u32, u32)>,
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
    };
    monsters.insert(open_spot, instance);
    filled_spots.insert(open_spot);

    instance
}

///
/// Finds a currently open spot on the map,
/// retrieving the current `dungeon`, and checking
/// current `monsters` and `players` positions,
/// filtering them out
///
fn open_spot<'a>(dungeon: &Dungeon, filled_spots: &'a HashSet<(u32, u32)>) -> (u32, u32) {
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
    filled_spots: &'a HashSet<(u32, u32)>,
    spot: (u32, u32),
) -> Option<&'a (u32, u32)> {
    dungeon
        .paths()
        .filter(|path| Transform::distance(**path, spot) <= 1.0)
        .filter(|path| !filled_spots.contains(path))
        .choose(&mut thread_rng())
}
