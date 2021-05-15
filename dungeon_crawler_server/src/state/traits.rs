use std::time::Instant;

use super::{
    stats::{Attributes, Stats},
    transforms::vec2::Vec2,
};

pub trait Identified {
    fn id(&self) -> u32;
}

///
/// Represents an entity that can move locations.
/// Required `Directed` (and with it, `Positioned`)
///
pub trait Translator: Identified {
    fn target(&self) -> Option<&Vec2>;
    fn set_path(&mut self, target: Vec<Vec2>);

    fn next_step(&mut self) -> Option<Vec2>;
}

pub trait Qualities {
    fn stats(&mut self) -> &mut Stats;
    fn attrs(&self) -> &Attributes;
}

pub trait Follower {
    fn follow_target(&self) -> Option<u32>;

    fn start_following(&mut self, id: u32);
    fn stop_following(&mut self);
    fn sight_range(&self) -> u32;

    fn last_sighting(&self) -> Instant;
    fn reset_last_sighting(&mut self);
}

pub trait Combater : Follower {
    fn charge_attk(&mut self) -> bool;
    fn reset_attk(&mut self);
}

pub enum AttackResult {
    Hit(u32, i32),
    Miss(u32),
}

pub trait AI: Translator + Combater {}
