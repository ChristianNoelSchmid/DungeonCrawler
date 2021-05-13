use std::fmt::Display;

use simple_serializer::Serialize;

use super::vec2::Vec2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == Direction::Left { 0 } else { 1 })
    }
}
impl Direction {
    pub fn from_u32(n: u32) -> Self {
        return match n {
            1 => Direction::Right,
            _ => Direction::Left,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec2,
    pub dir: Direction,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos: Vec2(0, 0),
            dir: Direction::Right,
        }
    }
    pub fn with_values(position: Vec2, direction: Direction) -> Self {
        Self {
            pos: position,
            dir: direction,
        }
    }
}

impl Serialize for Transform {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!("{}::{}::{}", self.pos.0, self.pos.1, self.dir)
    }
}
