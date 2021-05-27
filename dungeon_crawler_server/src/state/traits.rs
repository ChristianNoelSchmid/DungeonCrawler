use std::time::Instant;

use super::{
    stats::{Attributes, Stats},
    transforms::vec2::Vec2,
};

/// 
/// Represents an entity's ability to be
/// defined separately from all other entities.
///
pub trait Identified {
    /// A unique number which separates the
    /// Entity from all others
    fn id(&self) -> u32;
}

///
/// Represents an entity that can move locations.
/// Required `Directed` (and with it, `Positioned`)
///
pub trait Translator: Identified {
    /// A single progression towards implementing
    /// a single step. Returns true when the entity
    /// is ready to step forward.
    fn charge_step(&mut self) -> bool;
    /// The position the entity is moving towards
    fn target(&self) -> Option<&Vec2>;
    /// Creates a new path as a collection of `Vec2`s, which
    /// the entity progresses one at a time.
    fn set_path(&mut self, target: Vec<Vec2>);
    /// The next step the entity must take to reach
    /// its `target`
    fn next_step(&mut self) -> Option<Vec2>;
}

/// 
/// Defines an entity which has `Stats` and `Attributes`.
///
pub trait Qualities {
    fn stats(&mut self) -> &mut Stats;
    fn attrs(&self) -> &Attributes;
}

///
/// Defines an Entity which can lock onto
/// a different,`Identified` entity.
///
pub trait Follower {
    /// Returns the `id` of the entity's target,
    /// if the entity is currently following
    fn follow_target(&self) -> Option<u32>;

    /// Assigns a target to the entity, which it
    /// begins following.
    fn start_following(&mut self, id: u32);
    /// Stop following the target entity, if currently
    /// following one.
    fn stop_following(&mut self);
    /// Retreive the distance the entity can see
    fn sight_range(&self) -> u32;
    /// The last moment the entity saw the
    /// target entity.
    fn last_sighting(&self) -> Instant;
    /// Sets `last_sighting` to now
    fn reset_last_sighting(&mut self);
}

///
/// Defines an entity which can act in combat.
///
pub trait Combater: Follower {
    /// A single progression towards implementing
    /// a single attack. Returns true when the entity
    /// is ready to attempt an attack.
    fn charge_attk(&mut self) -> bool;
    /// Resets a attack's charge.
    fn reset_attk(&mut self);
}

pub enum AttackResult {
    Hit(u32, i32),
    Miss(u32),
}

pub trait AI: Translator + Combater {}
