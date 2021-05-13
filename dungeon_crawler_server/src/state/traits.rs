use std::time::Instant;

use super::transforms::vec2::Vec2;

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

pub trait Combater {
    fn start_combat_with(&mut self, id: u32);
    fn in_combat_with(&self) -> Option<u32>;
    fn stop_combat(&mut self);
    fn sight_range(&self) -> u32;

    fn last_sighting(&self) -> Instant;
    fn set_last_sighting(&mut self, last: Instant);
}

pub trait AI: Translator + Combater {}
