//! Definitions of Player structs
//!
//! Christian Schmid - June, 2021
//! CS510 - Programming Rust

use crate::state::{
    stats::{Attributes, Stats},
    traits::Identified,
};
use simple_serializer::Serialize;

use super::traits::Qualities;

///
/// Represents a Player in the StateManager
///
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,

    // Identified
    pub id: u32,

    // Qualities
    stats: Stats,
    attrs: Attributes,
}

impl Player {
    /// Creates a new Player with the given unique `id`
    /// and `name`
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

impl Qualities for Player {
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }

    fn attrs(&self) -> &Attributes {
        &self.attrs
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
