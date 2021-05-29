use std::str::FromStr;

use crate::state::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};
use simple_serializer::{Deserialize, Serialize};

///
/// Enum for all possible Event Types the EventManager
/// can parse.
///
#[derive(Debug)]
pub enum Type {
    Hello(String),        // a client joining the game                         (name)
    Welcome(u32, String), // info. the server relays to the client for syncing (id, dun_data)
    NewPlayer(u32, String, Vec2), // informs clients of a new Player,                  (id, name, pos)
    NewMonster(u32, u32, Vec2), // informs clients of a new Monster                  (temp_id, inst_id, pos)
    Moved(u32, Transform),      // informs server / clients of moved entity          (id, transform)
    PlayerLeft(u32),            // informs server / clients that a Player has left   (id)
    Charging(u32),
    AttemptHit(u32, u32),
    AttkTowards(u32, Vec2),
    Hit(u32, u32, i32), // informs clients that a Player has been hit        (attId, defId, healthLeft)
    Miss(u32, u32),     // informs clients that a Player has been missed     (attId, defId)
    Dead(u32),          // informs clients that a Player has died            (id)
    Escaped(u32),       // informs clients that a Player has escaped         (id)
    DungeonComplete,    // informs clients that the Dungeon has been completed
    Reconnect,          // requests that the clients reconnect - new StateManager
    Dropped,            // a dropped packet
}

impl Serialize for Type {
    type SerializeTo = String;
    fn serialize(&self) -> String {
        match self {
            Type::Hello(name) => format!("Hello::{}", name),
            Type::Welcome(id, dun) => format!("Welcome::{}::{}", id, dun),
            Type::NewPlayer(id, name, pos) => {
                format!("NewPlayer::{}::{}::{}", id, name, pos.serialize())
            }
            Type::NewMonster(t_id, i_id, pos) => {
                format!("NewMonster::{}::{}::{}", t_id, i_id, pos.serialize())
            }
            Type::Moved(id, transform) => format!("Moved::{}::{}", id, transform.serialize()),
            Type::PlayerLeft(id) => format!("PlayerLeft::{}", id),
            Type::AttkTowards(id, pos) => format!("AttkTowards::{}::{}", id, pos.serialize()),
            Type::Charging(id) => format!("Charging::{}", id),
            Type::AttemptHit(attk_id, defd_id) => format!("AttemptHit::{}::{}", attk_id, defd_id),
            Type::Hit(att_id, def_id, cur_health) => {
                format!("Hit::{}::{}::{}", att_id, def_id, cur_health)
            }
            Type::Miss(att_id, def_id) => format!("Miss::{}::{}", att_id, def_id),
            Type::Dead(id) => format!("Dead::{}", id),
            Type::Escaped(id) => format!("Escaped::{}", id),
            Type::DungeonComplete => "DungeonComplete::".to_string(),
            Type::Reconnect => "Reconnect::".to_string(),
            Type::Dropped => "Drop".to_string(),
        }
    }
}

impl Deserialize for Type {
    type DeserializeTo = Type;

    fn deserialize(from: &str) -> Self::DeserializeTo {
        let segs: Vec<&str> = from.split("::").collect();

        match segs[0].trim() {
            "Hello" => Type::Hello(segs[1].to_string()),
            "Left" => {
                if let Ok(id) = u32::from_str(segs[1].trim()) {
                    Type::PlayerLeft(id)
                } else {
                    Type::Dropped
                }
            }
            "Moved" => {
                match (
                    u32::from_str(segs[1]),
                    i32::from_str(segs[2]),
                    i32::from_str(segs[3]),
                    u32::from_str(segs[4]),
                ) {
                    (Ok(id), Ok(x), Ok(y), Ok(d)) => Type::Moved(
                        id,
                        Transform::with_values(Vec2(x, y), Direction::from_u32(d)),
                    ),
                    _ => Type::Dropped,
                }
            },
            "AttemptHit" => {
                match(
                    u32::from_str(segs[1]),
                    u32::from_str(segs[2]),
                ) {
                    (Ok(attk_id), Ok(defd_id)) => Type::AttemptHit(attk_id, defd_id),
                    _ => Type::Dropped,
                }
            },
            _ => Type::Dropped,
        }
    }
}
