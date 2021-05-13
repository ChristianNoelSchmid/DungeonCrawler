use std::collections::{HashMap, HashSet};

use rand::{prelude::IteratorRandom, thread_rng};

use crate::state::actor::{Actor, ActorId};

use super::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

#[derive(Clone, Debug)]

pub struct WorldStage {
    actors: HashMap<u32, Actor>,
    paths: HashSet<Vec2>,
    filled_spots: HashSet<Vec2>,
}

impl WorldStage {
    pub fn new(paths: HashSet<Vec2>) -> Self {
        Self {
            actors: HashMap::new(),
            paths: paths,
            filled_spots: HashSet::new(),
        }
    }
    pub fn actor<'a>(&'a self, id: u32) -> Option<&'a Actor> {
        self.actors.get(&id)
    }
    pub fn clone_transforms<'a>(&'a self) -> Vec<(u32, Transform)> {
        self.actors
            .clone()
            .into_iter()
            .map(|a| (a.0, a.1.tr))
            .collect()
    }
    pub fn add(&mut self, id: u32, actor: Actor) -> Option<&Actor> {
        if !self.actors.contains_key(&id) {
            self.actors.insert(id, actor);

            return Some(&self.actors[&id]);
        }
        None
    }
    pub fn remove(&mut self, id: u32) -> bool {
        self.actors.remove(&id).is_some()
    }
    pub fn from_transform(&mut self, id: u32, new_t: Transform) -> Option<Transform> {
        self.move_pos(id, new_t.pos);
        if let Some(a) = self.actors.get_mut(&id) {
            a.tr.dir = new_t.dir;
            return Some(a.tr);
        }
        None
    }
    pub fn pos(&self, id: u32) -> Option<Vec2> {
        return if let Some(a) = self.actors.get(&id) {
            Some(a.tr.pos)
        } else {
            None
        };
    }
    pub fn dir(&self, id: u32) -> Option<Direction> {
        return if let Some(a) = self.actors.get(&id) {
            Some(a.tr.dir)
        } else {
            None
        };
    }
    pub fn move_pos(&mut self, id: u32, new_pos: Vec2) -> bool {
        if let Some(a) = self.actors.get_mut(&id) {
            if !self.filled_spots.contains(&new_pos) && self.paths.contains(&new_pos) {
                self.filled_spots.remove(&a.tr.pos);
                self.filled_spots.insert(new_pos);

                if new_pos.0 > a.tr.pos.0 {
                    a.tr.dir = Direction::Right;
                } else if new_pos.0 < a.tr.pos.0 {
                    a.tr.dir = Direction::Left;
                }

                a.tr.pos = new_pos;

                return true;
            }
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

    pub fn is_spot_open(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot) && !self.filled_spots.contains(&spot)
    }

    pub fn is_actor_id_on_spot(&self, actor_id: ActorId, spot: Vec2) -> Option<&Actor> {
        self.actors
            .values()
            .filter(|a| a.actor_id == actor_id && a.tr.pos == spot)
            .next()
    }

    pub fn is_on_paths(&self, spot: Vec2) -> bool {
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
                    Vec2::distance(**path, a.tr.pos) <= range as f32
                        && !self.filled_spots.contains(path)
                })
                .choose(&mut thread_rng())
            {
                return Some(*spot);
            }
        }
        None
    }
}
