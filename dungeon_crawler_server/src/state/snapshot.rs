use simple_serializer::Serialize;

use super::{monsters::MonsterInstance, players::Player};
use std::net::SocketAddr;

pub struct StateSnapshot {
    pub addr_for: SocketAddr,
    pub player_id: u32,
    pub players: Vec<Player>,
    pub monsters: Vec<MonsterInstance>,
    pub paths: String,
    pub entrance: (i32, i32),
    pub exit: (i32, i32),
}
