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
    fn sight_range(&self) -> u32;
}

pub trait AI: Translator + Combater {}
