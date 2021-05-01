use std::net::SocketAddr;

use super::{monsters::MonsterInstance, snapshot::StateSnapshot, transform::Transform};

pub enum RequestType {
    NewPlayer(SocketAddr, u32),
    PlayerMoved(u32, Transform),
    SpawnMonster(u32),
}

pub enum ResponseType {
    StateSnapshot(StateSnapshot),
    NewMonster(MonsterInstance),
    MonsterMoved(MonsterInstance),
}
