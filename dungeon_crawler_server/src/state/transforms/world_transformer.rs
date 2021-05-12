use std::collections::{HashMap, HashSet};

use rand::{prelude::IteratorRandom, thread_rng};

use super::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

#[derive(Clone, Debug)]

pub struct WorldTransformer {
    transforms: HashMap<u32, Transform>,
    paths: HashSet<Vec2>,
    filled_spots: HashSet<Vec2>,
}

impl WorldTransformer {
    pub fn new(paths: HashSet<Vec2>) -> Self {
        Self {
            transforms: HashMap::new(),
            paths: paths,
            filled_spots: HashSet::new(),
        }
    }
    pub fn transform<'a>(&'a self, id: u32) -> Option<&'a Transform> {
        self.transforms.get(&id)
    }
    pub fn clone_transforms<'a>(&'a self) -> Vec<(u32, Transform)> {
        self.transforms.clone().into_iter().collect()
    }
    pub fn add(&mut self, id: u32, new_t: Transform) -> Option<Transform> {
        if !self.transforms.contains_key(&id) {
            self.transforms.insert(id, new_t);

            return Some(new_t);
        }
        None
    }
    pub fn remove(&mut self, id: u32) -> bool {
        self.transforms.remove(&id).is_some()
    }
    pub fn from_transform(&mut self, id: u32, new_t: Transform) -> Option<Transform> {
        if let Some(t) = self.transforms.get_mut(&id) {
            *t = new_t;
            return Some(*t);
        }
        None
    }
    pub fn pos(&self, id: u32) -> Option<Vec2> {
        return if let Some(t) = self.transforms.get(&id) {
            Some(t.position)
        } else {
            None
        };
    }
    pub fn dir(&self, id: u32) -> Option<Direction> {
        return if let Some(t) = self.transforms.get(&id) {
            Some(t.direction)
        } else {
            None
        };
    }
    pub fn move_pos(&mut self, id: u32, new_pos: Vec2) -> bool {
        if let Some(t) = self.transforms.get_mut(&id) {
            if !self.filled_spots.contains(&new_pos) && self.paths.contains(&new_pos) {
                self.filled_spots.remove(&t.position);
                self.filled_spots.insert(new_pos);

                if new_pos.0 > t.position.0 {
                    t.direction = Direction::Right;
                } else if new_pos.1 < t.position.1 {
                    t.direction = Direction::Left;
                }

                t.position = new_pos;

                return true;
            }
        }

        false
    }
    pub fn change_dir(&mut self, id: u32, new_dir: Direction) -> Option<Transform> {
        if let Some(t) = self.transforms.get_mut(&id) {
            t.direction = new_dir;
            return Some(*t);
        }
        None
    }

    pub fn is_spot_open(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot) && !self.filled_spots.contains(&spot)
    }

    pub fn is_on_paths(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot)
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
        if let Some(transform) = self.transforms.get(&id) {
            if let Some(spot) = self
                .paths
                .iter()
                .filter(|path| {
                    Vec2::distance(**path, transform.position) <= range as f32
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
