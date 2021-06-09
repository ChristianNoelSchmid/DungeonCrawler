//! Actor type for World Stage in
//! Dungeon State Manager
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

use simple_serializer::Serialize;

use crate::state::transforms::transform::Transform;

use super::{
    stats::{Attributes, Stats},
    traits::Qualities,
};

/// 
/// Defines the status of an `Actor` - whether
/// they are `Active`, `Dead`, or `Escaped` (from the
/// `Dungeon`)
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Dead,
    Escaped,
}

impl Serialize for Status {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!(
            "{}",
            match self {
                Status::Active => 0,
                Status::Dead => 1,
                Status::Escaped => 2,
            }
        )
    }
}

///
/// The definitions for an entity in a
/// `WorldStage`. Contains all information
/// needed to transform and act / interact
/// with other `Actor`s.
///
#[derive(Debug, Clone)]
pub struct Actor {
    pub id: u32,
    pub stats: Stats,
    pub attrs: Attributes,
    pub tr: Transform,
    pub actor_id: ActorId,
    pub status: Status,
}

impl Actor {
    pub fn new(id: u32, stats: Stats, attrs: Attributes, tr: Transform, actor_id: ActorId) -> Self {
        Self {
            id,
            stats,
            attrs,
            tr,
            actor_id,
            status: Status::Active,
        }
    }
}

// Implementation of Qualities trait for Actor
impl Qualities for Actor {
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }
    fn attrs(&self) -> &Attributes {
        &self.attrs
    }
}

/// A collection of all possible `Actor` affiliations.
/// `Actor`s can take on various affiliations, which will
/// impact interaction with other `Actor`s on the `WorldStage`.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ActorId {
    Player,
    Monster,
}

/// A list of all possible `Actor` types available
/// for the server to use.
const ACTOR_IDS: [ActorId; 2] = [ActorId::Player, ActorId::Monster];

impl ActorId {
    /// Simplified way to select all `ActorId`s except
    /// the one specified by `actor_id`.
    pub fn all_but(actor_id: ActorId) -> Vec<ActorId> {
        ACTOR_IDS
            .iter()
            .filter(|id| *id != &actor_id)
            .cloned()
            .collect()
    }
}
