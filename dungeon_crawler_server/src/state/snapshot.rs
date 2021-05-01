use simple_serializer::Serialize;

use super::{monsters::MonsterInstance, players::Player};
use std::net::SocketAddr;

pub struct StateSnapshot {
    pub addr_for: SocketAddr,
    pub players: Vec<Player>,
    pub monsters: Vec<MonsterInstance>,
    pub paths: String,
    pub entrance: (u32, u32),
    pub exit: (u32, u32),
}
