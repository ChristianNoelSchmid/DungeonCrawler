
use simple_serializer::Serialize;

use super::transform::{Transform};

///
/// Represents a Player in the state
///
#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub transform: Transform,
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
