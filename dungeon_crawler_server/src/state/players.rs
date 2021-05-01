use simple_serializer::Serialize;

use super::transform::{Direction, Transform};

///
/// Represents a Player in the state
///
#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub transform: Transform,
}

impl Player {
    pub fn position(&mut self, pos: (u32, u32)) {
        self.transform.position = pos;
    }
    pub fn direction(&mut self, dir: Direction) {
        self.transform.direction = dir;
    }
}

///
/// Serialization for the Player
/// (To String)
///
impl Serialize for Player {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!("{}::{}", self.id, self.transform.serialize())
    }
}
