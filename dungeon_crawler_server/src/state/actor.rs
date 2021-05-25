use simple_serializer::Serialize;

use crate::state::transforms::transform::Transform;

use super::{
    stats::{Attributes, Stats},
    traits::Qualities,
};

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

#[derive(Debug, Clone)]
pub struct Actor {
    pub stats: Stats,
    pub attrs: Attributes,
    pub id: u32,
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

impl Qualities for Actor {
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }
    fn attrs(&self) -> &Attributes {
        &self.attrs
    }
}

const ACTOR_IDS: [ActorId; 2] = [ActorId::Player, ActorId::Monster];

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ActorId {
    Player,
    Monster,
}

impl ActorId {
    pub fn all_but<'a>(actor_id: ActorId) -> Vec<ActorId> {
        ACTOR_IDS
            .iter()
            .filter(|id| *id != &actor_id)
            .cloned()
            .collect()
    }
}
