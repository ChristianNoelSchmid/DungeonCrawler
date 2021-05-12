use crate::state::{
    stats::{Attributes, Stats},
    traits::Identified,
};
use simple_serializer::Serialize;

///
/// Represents a Player in the state
///
#[derive(Debug, Clone)]
pub struct Player {
    stats: Stats,
    attrs: Attributes,
    pub id: u32,
    pub name: String,
}

impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            stats: Stats::new(20, 10, 10),
            attrs: Attributes::new(4, 4, 4),
            id,
            name,
        }
    }
}

impl Identified for Player {
    fn id(&self) -> u32 {
        self.id
    }
}

///
/// Serialization for the Player
/// (To String)
///
impl Serialize for Player {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!("{}", self.id)
    }
}
