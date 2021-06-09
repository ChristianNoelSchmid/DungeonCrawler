//! Snapshot of current State
//!
//! Christian Schmid - June, 2021
//! CS510 - Programming Rust

use dungeon_generator::inst::Dungeon;

use std::net::SocketAddr;

use super::transforms::{transform::Transform, vec2::Vec2};

///
/// Represents a snapshot in time of the current state of a
/// `StateManager`. Used to synchronize a new `Player` with the
/// current state of the game
///
pub struct StateSnapshot {
    pub addr_for: SocketAddr,
    pub new_player: (u32, String, Vec2),
    pub other_players: Vec<(u32, String, Vec2, String)>,
    pub all_player_ts: Vec<(u32, Transform)>,
    pub monsters: Vec<(u32, u32, Vec2)>,
    pub dungeon: Dungeon,
}
