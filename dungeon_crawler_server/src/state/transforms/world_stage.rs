use std::collections::{HashMap, HashSet};

use crossbeam::channel::Sender;
use rand::{prelude::IteratorRandom, thread_rng, RngCore};

use crate::state::{
    actor::{Actor, ActorId},
    traits::Qualities,
    types::ResponseType,
};

use super::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

#[derive(Clone, Debug)]

pub struct WorldStage {
    actors: HashMap<u32, Actor>,
    paths: HashSet<Vec2>,
    filled_spots: HashSet<Vec2>,

    s_to_event: Sender<ResponseType>,
}

impl WorldStage {
    pub fn new(paths: HashSet<Vec2>, s_to_event: Sender<ResponseType>) -> Self {
        Self {
            actors: HashMap::new(),
            paths,
            filled_spots: HashSet::new(),
            s_to_event,
        }
    }
    pub fn actor(&mut self, id: u32) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }
    pub fn clone_transforms(&self) -> Vec<(u32, Transform)> {
        self.actors
            .clone()
            .into_iter()
            .map(|a| (a.0, a.1.tr))
            .collect()
    }
    pub fn add(&mut self, id: u32, actor: Actor) -> Option<&Actor> {
        match self.actors.get(&id) {
            Some(_a) => None,
            None => {
                self.actors.insert(id, actor);
                Some(&self.actors[&id])
            }
        }
    }
    pub fn remove(&mut self, id: u32) -> bool {
        self.actors.remove(&id).is_some()
    }
    pub fn update_transform(&mut self, id: u32, new_t: Transform) -> Option<Transform> {
        if !self.is_spot_open(new_t.pos) {
            return None;
        }
        if let Some(act) = self.actors.get_mut(&id) {
            self.filled_spots.remove(&act.tr.pos);
            self.filled_spots.insert(new_t.pos);

            act.tr = new_t;
            return Some(act.tr);
        }
        None
    }
    pub fn pos(&self, id: u32) -> Option<Vec2> {
        self.actors.get(&id).map(|a| a.tr.pos)
    }
    pub fn dir(&self, id: u32) -> Option<Direction> {
        self.actors.get(&id).map(|a| a.tr.dir)
    }
    pub fn move_pos(&mut self, id: u32, new_pos: Vec2) -> bool {
        if !self.is_spot_open(new_pos) {
            return false;
        }
        if let Some(a) = self.actors.get_mut(&id) {
            self.filled_spots.remove(&a.tr.pos);
            self.filled_spots.insert(new_pos);

            a.tr.dir = match new_pos.0 {
                p if p > a.tr.pos.0 => Direction::Right,
                p if p < a.tr.pos.0 => Direction::Left,
                _ => a.tr.dir,
            };

            a.tr.pos = new_pos;

            self.s_to_event
                .send(ResponseType::MonsterMoved(id, a.tr))
                .unwrap();

            return true;
        }

        false
    }
    pub fn change_dir(&mut self, id: u32, new_dir: Direction) -> Option<Transform> {
        if let Some(a) = self.actors.get_mut(&id) {
            a.tr.dir = new_dir;
            return Some(a.tr);
        }
        None
    }

    pub fn look_at(&mut self, id: u32, pos: Vec2) -> bool {
        if let Some(actor) = self.actors.get_mut(&id) {
            let mut dir = actor.tr.dir;
            if actor.tr.pos.0 < pos.0 {
                dir = Direction::Right;
            } else if actor.tr.pos.0 > pos.0 {
                dir = Direction::Left;
            }

            actor.tr.dir = dir;
            return true;
        }
        false
    }

    pub fn is_spot_open(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot) && !self.filled_spots.contains(&spot)
    }

    pub fn is_actor_id_on_spot(&self, actor_id: ActorId, spot: Vec2) -> Option<&Actor> {
        self.actors
            .values()
            .find(|a| a.actor_id == actor_id && a.tr.pos == spot)
    }

    pub fn is_on_path(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot)
    }

    pub fn is_actor_on_spot(&self, id: u32, spot: Vec2) -> bool {
        self.actors[&id].tr.pos == spot
    }

    ///
    /// Finds a currently open spot on the map,
    /// retrieving the current `dungeon`, and checking
    /// current `monsters` and `players` positions,
    /// filtering them out
    ///
    pub fn open_spot(&self) -> Vec2 {
        *self
            .paths
            .iter()
            .filter(|path| !self.filled_spots.contains(path))
            .choose(&mut thread_rng())
            .unwrap()
    }

    ///
    /// Finds a currently open spot on the map,
    /// retrieving the current `dungeon`, and checking
    /// current `monsters` and `players` positions,
    /// filtering them out
    ///
    pub fn open_spot_within(&self, id: u32, range: u32) -> Option<Vec2> {
        if let Some(a) = self.actors.get(&id) {
            if let Some(spot) = self
                .paths
                .iter()
                .filter(|path| {
                    path.distance(a.tr.pos) <= range as f32 && !self.filled_spots.contains(path)
                })
                .choose(&mut thread_rng())
            {
                return Some(*spot);
            }
        }
        None
    }

    pub fn attk(&mut self, attk_id: u32, defd_id: u32) {
        let attacker = &self.actors[&attk_id];
        let attk_damage = attacker.attrs.might as i32;

        let defender = &self.actors[&defd_id];

        if thread_rng().next_u32() % 100 > defender.attrs().fines {
            let health = &mut self.actors.get_mut(&defd_id).unwrap().stats().cur_health;
            *health -= attk_damage;

            self.s_to_event
                .send(ResponseType::Hit(attk_id, defd_id, *health))
                .unwrap();
        } else {
            self.s_to_event
                .send(ResponseType::Miss(attk_id, defd_id))
                .unwrap();
        }
    }
}
