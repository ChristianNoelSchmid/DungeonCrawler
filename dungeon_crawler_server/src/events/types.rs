use std::str::FromStr;

use crate::state::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};
use simple_serializer::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Type {
    Hello(String),
    Welcome(u32, String),         // id, dungeon paths
    NewPlayer(u32, String, Vec2), // id, (x, y)
    NewMonster(u32, u32, Vec2),   // template_id, instance_id, pos
    Moved(u32, Transform),        // id, transform
    PlayerLeft(u32),              // id
    Hit(u32, u32, i32),
    Miss(u32, u32),
    Dead(u32),
    Escaped(u32),
    DungeonComplete,
    Reconnect,
    Dropped,
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
            Type::Hit(att_id, def_id, cur_health) => {
                format!("Hit::{}::{}::{}", att_id, def_id, cur_health)
            }
            Type::Miss(att_id, def_id) => format!("Miss::{}::{}", att_id, def_id),
            Type::Dead(id) => format!("Dead::{}", id),
            Type::Escaped(id) => format!("Escaped::{}", id),
            Type::DungeonComplete => format!("DungeonComplete::"),
            Type::Reconnect => format!("Reconnect::"),
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
            }
            _ => Type::Dropped,
        }
    }
}
