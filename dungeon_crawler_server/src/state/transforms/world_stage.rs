//! World Stage for all Actors in State Manager
//!
//! Christian Schmid - May, 2021
//! CS510 - Programming Rust


use std::collections::{HashMap, HashSet};

use crossbeam::channel::Sender;
use rand::{prelude::IteratorRandom, thread_rng, RngCore};

use crate::state::{
    actor::{Actor, ActorId, Status},
    traits::Qualities,
    types::ResponseType,
};

use super::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

///
/// A representation of the entire physical layout of the level,
/// with Actor data.
///
#[derive(Clone, Debug)]

pub struct WorldStage {
    // All Actors keyed by their Id
    actors: HashMap<u32, Actor>,
    // All paths available
    paths: HashSet<Vec2>,
    // The position of the entrance
    entrance: Vec2,
    // The position of the exit
    exit: Vec2,
    // All spots currently filled by actors
    filled_spots: HashSet<Vec2>,

    // A clone of the Sender to the EventManager
    s_to_event: Sender<ResponseType>,
}

impl WorldStage {
    /// Creates a new `WorldStage` with the specified `paths`,
    /// `entrance` and `exit` points, and `Sender` `s_to_event`.
    pub fn new(
        paths: HashSet<Vec2>,
        entrance: Vec2,
        exit: Vec2,
        s_to_event: Sender<ResponseType>,
    ) -> Self {
        Self {
            actors: HashMap::new(),
            paths,
            entrance,
            exit,
            filled_spots: HashSet::new(),
            s_to_event,
        }
    }
    /// Retrieves an `Actor` from WorldStage via 'id', if the `Actor` exists
    pub fn actor(&mut self, id: u32) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }
    /// Clones all `Transform`s in the `WorldStage`, returning a Vec
    /// with each `Transform` and their id.
    pub fn clone_transforms(&self) -> Vec<(u32, Transform)> {
        self.actors
            .clone()
            .into_iter()
            .map(|a| (a.0, a.1.tr))
            .collect()
    }
    /// Adds a new `actor`, with the given `id`, if it doesn't
    /// already exist. Returns `None` if the actor *does* already exist.
    pub fn add(&mut self, id: u32, actor: Actor) -> Option<&Actor> {
        match self.actors.get(&id) {
            Some(_a) => None,
            None => {
                self.actors.insert(id, actor);
                Some(&self.actors[&id])
            }
        }
    }
    /// Removes an `Actor` by its `id`. Returns
    /// `true` if the Actor exists, or else `false`.
    pub fn remove(&mut self, id: u32) -> bool {
        self.actors.remove(&id).is_some()
    }
    /// Updates a Player's transform by `id`, using `Transform` `new_t`.
    /// Does **not** send the change to the `EventManager`, as that's handled
    /// separately. Returns false if the position of `new_t` is already filled.
    pub fn update_pl_tr(&mut self, id: u32, new_t: Transform) -> bool {
        // If the spot is filled, return false
        if !self.is_spot_open(new_t.pos) {
            false
        }
        // If the Actor exists, move it's position and update the WorldStage
        // accordingly.
        else if let Some(act) = self.actors.get_mut(&id) {
            // Move the filled spot
            self.filled_spots.remove(&act.tr.pos);
            self.filled_spots.insert(new_t.pos);

            // Set the Actor's transform
            act.tr = new_t;

            // If the new position is the exit, send an Escaped
            // update to the clients, and update the Actor status
            if act.tr.pos == self.exit {
                act.status = Status::Escaped;
                self.filled_spots.remove(&act.tr.pos);
                self.s_to_event.send(ResponseType::Escaped(act.id)).unwrap();
            }

            true
        } else {
            false
        }
    }
    /// Retrieves the position of a given `Actor`, by `id`,
    /// if the `Actor` exists.
    pub fn pos(&self, id: u32) -> Option<Vec2> {
        self.actors.get(&id).map(|a| a.tr.pos)
    }
    /// Retrieves the direction of a given `Actor`, by `id`,
    /// if the `Actor` exists.
    pub fn dir(&self, id: u32) -> Option<Direction> {
        self.actors.get(&id).map(|a| a.tr.dir)
    }
    /// Updates the position of an `Actor` with the given `id`,
    /// with `new_pos`, if the Actor exists.
    pub fn move_pos(&mut self, id: u32, new_pos: Vec2) -> bool {
        // If the spot is filled, return false
        if !self.is_spot_open(new_pos) {
            return false;
        }
        // If the Actor exists, move it's position and update the WorldStage
        // accordingly.
        if let Some(a) = self.actors.get_mut(&id) {
            // Move the filled spot
            self.filled_spots.remove(&a.tr.pos);
            self.filled_spots.insert(new_pos);

            // Set Actor's direction to match the direction they are moving.
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
    /// Updates the position of an `Actor` with the given `id`,
    /// with `new_dir`, if the Actor exists.
    pub fn change_dir(&mut self, id: u32, new_dir: Direction) -> bool {
        if let Some(a) = self.actors.get_mut(&id) {
            a.tr.dir = new_dir;
            true
        } else {
            false
        }
    }
    /// Directs the `Actor` with the given `id` to look towards
    /// the given `pos`.
    pub fn look_at(&mut self, id: u32, pos: Vec2) -> bool {
        if let Some(actor) = self.actors.get_mut(&id) {
            let mut dir = actor.tr.dir;
            dir = match actor.tr.pos.0 {
                x if x < pos.0 => Direction::Right,
                x if x > pos.0 => Direction::Left,
                _ => dir,
            };

            actor.tr.dir = dir;
            return true;
        }
        false
    }

    /// Tests if the given `spot` is open
    pub fn is_spot_open(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot) && !self.filled_spots.contains(&spot)
    }

    /// Tests if the `actor_id` is at the position `spot`.
    pub fn is_actor_id_on_spot(&self, actor_id: ActorId, spot: Vec2) -> Option<&Actor> {
        self.actors
            .values()
            .find(|a| a.actor_id == actor_id && a.tr.pos == spot && a.status == Status::Active)
    }

    /// Tests if the given `spot` is on a path
    pub fn is_on_path(&self, spot: Vec2) -> bool {
        self.paths.contains(&spot)
    }

    /// Tests if the `Actor` with the given `id` is at position `spot`
    pub fn is_actor_on_spot(&self, id: u32, spot: Vec2) -> bool {
        self.actors[&id].tr.pos == spot && self.actors[&id].status == Status::Active
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
            .filter(|path| path.distance(self.entrance) > 15.0)
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

    /// Performs an 'attack' from the `attk_id` `Actor` to `defd_id` `Actor`.
    pub fn attk(&mut self, attk_id: u32, defd_id: u32) {
        // Retrieve the attacker and defender Actors
        // and the attacker's attack damage (might stat)
        let attacker = &self.actors[&attk_id];
        let attk_damage = attacker.attrs.might as i32;

        let defender = &mut self.actors.get_mut(&defd_id).unwrap();

        // Test if the defender dodges the attack. If so, return a Miss.
        // Otherwise, return a Hit, with the defender's remaining health.
        if thread_rng().next_u32() % 100 > defender.attrs().fines {
            let health = &mut defender.stats().cur_health;
            *health -= attk_damage;

            // If health is below 1, set defender's status to dead,
            // and inform the EventManager.
            if *health <= 0 {
                defender.status = Status::Dead;
                self.filled_spots.remove(&defender.tr.pos);
                self.s_to_event.send(ResponseType::Dead(defd_id)).unwrap();
            }

            self.s_to_event
                .send(ResponseType::Hit(
                    attk_id,
                    defd_id,
                    defender.stats().cur_health,
                ))
                .unwrap();
        } else {
            self.s_to_event
                .send(ResponseType::Miss(attk_id, defd_id))
                .unwrap();
        }
    }
}
