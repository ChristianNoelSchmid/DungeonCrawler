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
    pub position: Vec2,
    pub direction: Direction,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec2(0, 0),
            direction: Direction::Right,
        }
    }
    pub fn with_values(position: Vec2, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl Serialize for Transform {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!(
            "{}::{}::{}",
            self.position.0, self.position.1, self.direction
        )
    }
}
