use std::net::SocketAddr;

use crate::state::snapshot::StateSnapshot;

use super::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

///
/// All requests that can be sent to the `StateManager`
/// which it parses and performs changes in state on.
///
pub enum RequestType {
    NewPlayer(SocketAddr, u32, String), // (client address, id, name)
    DropPlayer(u32),                    // (id)
    PlayerMoved(u32, Transform),        // (id, player transform)
    SpawnMonster(u32),                  // (id)
    Abort,
}

///
/// All responses that the `StateManager` can send
/// which are used to synchronize clients to the game
///
pub enum ResponseType {
    StateSnapshot(StateSnapshot),
    NewMonster(u32, u32, Vec2, Direction), // (temp_id, inst_id, pos, dir)
    MonsterMoved(u32, Transform),          // (inst_id, transform)
    Charging(u32),
    Hit(u32, u32, i32),                    // (attk_id, defd_id, defd health left)
    Miss(u32, u32),                        // (attk_id, defd_id)
    Dead(u32),                             // (id)
    Escaped(u32),                          // (id)
    DungeonComplete,
}
