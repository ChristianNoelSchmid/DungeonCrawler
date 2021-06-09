//! Transform struct for Actor movement in State Manager
//!
//! Christian Schmid - May, 2021
//! CS510 - Programming Rust

use std::fmt::Display;

use simple_serializer::Serialize;

use super::vec2::Vec2;

///
/// Represents an Entity's facing direction
///
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
        match n {
            1 => Direction::Right,
            _ => Direction::Left,
        }
    }
}

///
/// An Entity's Position and Direction
///
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec2,
    pub dir: Direction,
}

impl Transform {
    /// Creates a new `Transform` with default pos and dir values
    pub fn new() -> Self {
        Self {
            pos: Vec2(0, 0),
            dir: Direction::Right,
        }
    }
    /// Creates a new `Transform` with specified values
    pub fn with_values(position: Vec2, direction: Direction) -> Self {
        Self {
            pos: position,
            dir: direction,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Transform {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!("{}::{}::{}", self.pos.0, self.pos.1, self.dir)
    }
}
