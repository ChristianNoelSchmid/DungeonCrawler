use crate::state::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

#[derive(Debug, Clone)]
pub struct Actor {
    pub id: u32,
    pub tr: Transform,
    pub actor_id: ActorId,
}

impl Actor {
    pub fn new(id: u32, pos: Vec2, dir: Direction, actor_id: ActorId) -> Self {
        Self {
            id,
            tr: Transform { pos, dir },
            actor_id,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActorId {
    Player,
    Monster,
}
