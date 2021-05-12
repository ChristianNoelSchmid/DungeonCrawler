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
    fn attk(&self);
}

pub trait AI: Translator + Combater {}
