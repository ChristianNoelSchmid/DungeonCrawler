use std::{collections::HashSet, fmt::Display};

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
    position: (i32, i32),
    direction: Direction,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::Right,
        }
    }
    pub fn with_values(position: (i32, i32), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn pos(&self) -> (i32, i32) {
        self.position
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn from_other(&mut self, filled_spots: &mut HashSet<(i32, i32)>, other: Transform) -> bool {
        self.change_dir(other.direction);
        return self.move_pos(filled_spots, other.position);
    }
    pub fn move_pos(&mut self, filled_spots: &mut HashSet<(i32, i32)>, new_pos: (i32, i32)) -> bool {
        if filled_spots.contains(&new_pos) {
            return false;
        }
        filled_spots.remove(&self.position);
        self.position = new_pos;
        filled_spots.insert(new_pos);
        true
    }
    pub fn change_dir(&mut self, new_dir: Direction) {
        self.direction = new_dir
    }
    pub fn distance(spot1: (i32, i32), spot2: (i32, i32)) -> f32 {
        f32::sqrt((spot1.0 - spot2.0).pow(2) as f32 + (spot1.1 - spot2.1).pow(2) as f32)
    }
    pub fn add_pos(spot1: (i32, i32), spot2: (i32, i32)) -> (i32, i32) {
        (spot1.0 + spot2.0, spot1.1 + spot2.1)
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
