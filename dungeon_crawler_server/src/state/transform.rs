use std::{fmt::Display, str::FromStr};

use simple_serializer::Serialize;

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
    pub position: (u32, u32),
    pub direction: Direction,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::Right,
        }
    }
    pub fn with_values(position: (u32, u32), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn distance(spot1: (u32, u32), spot2: (u32, u32)) -> f32 {
        f32::sqrt(
            (spot1.0 as i32 - spot2.0 as i32).pow(2) as f32
                + (spot1.1 as i32 - spot2.1 as i32).pow(2) as f32,
        )
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
