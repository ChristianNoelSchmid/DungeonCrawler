use std::net::SocketAddr;

use crate::state::snapshot::StateSnapshot;

use super::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

pub enum RequestType {
    NewPlayer(SocketAddr, u32),
    DropPlayer(u32),
    PlayerMoved(u32, Transform),
    SpawnMonster(u32),
    AStar(Vec2),
}

pub enum ResponseType {
    StateSnapshot(StateSnapshot),
    NewMonster(u32, u32, Vec2, Direction),
    MonsterMoved(u32, Transform),
    Hit(u32, u32, i32),
    Miss(u32, u32),
    Dead(u32),
    Escaped(u32),
}
