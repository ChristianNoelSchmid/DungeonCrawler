use super::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

///
/// Represents being positioned on the map
/// by Vec2 location. Without `Directed` and
/// `Translater`, a `Positioned` entity can only
/// be located at a particular point.
///
pub trait Positioned {
    fn pos(&self) -> Vec2;
}

///
/// Represents an entity directed either left or right.
/// Requires being `Positioned`. `Direction` can be
/// mutated.
///
pub trait Directed: Positioned {
    fn dir(&self) -> Direction;
    fn face_dir(&mut self, dir: Direction);
}

///
/// Represents an entity that can move locations.
/// Required `Directed` (and with it, `Positioned`)
///
pub trait Translater: Directed {
    fn move_pos(&mut self, new_pos: Vec2) -> bool;
    fn change_trans(&mut self, new_t: Transform) -> bool;
    fn spot_within(&self, range: u32) -> Option<&Vec2>;
}

///
/// Represents an entity that can follow a path
/// to a specified `target` location.
///
pub trait TargetTranslator: Translater {
    fn target(&self) -> Option<&Vec2>;
    fn next_to_target(&self) -> bool;
    fn move_next(&mut self);
    fn set_target(&mut self, target: Vec2);
}

pub trait Combater: Positioned {
    fn attk(&self);
}

pub trait AI: TargetTranslator + Combater {}
