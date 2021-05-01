use std::str::FromStr;

use crate::state::{
    monsters::MonsterInstance,
    players::Player,
    transform::{Direction, Transform},
};
use simple_serializer::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Type<'a, 'b> {
    Hello,
    Welcome(u32, String),            // id, dungeon paths
    NewPlayer(&'a Player),           // id, (x, y)
    NewMonster(&'b MonsterInstance), // template_id, instance_id, (x, y)
    Moved(u32, Transform),           // id, transform
    Left(u32),                       // id
    Dropped,
}

impl<'a, 'b> Serialize for Type<'a, 'b> {
    type SerializeTo = String;
    fn serialize(&self) -> String {
        match self {
            Type::Hello => "Hello".to_string(),
            Type::Welcome(id, dun) => format!("Welcome::{}::{}", id, dun),
            Type::NewPlayer(player) => format!("NewPlayer::{}", player.serialize()),
            Type::NewMonster(monster) => {
                format!("NewMonster::{}", monster.serialize())
            }
            Type::Moved(id, transform) => format!("Moved::{}::{}", id, transform.serialize()),
            Type::Left(id) => format!("Left::{}", id),
            Type::Dropped => "Drop".to_string(),
        }
    }
}

impl<'a, 'b> Deserialize for Type<'a, 'b> {
    type DeserializeTo = Type<'a, 'b>;

    fn deserialize(from: &str) -> Self::DeserializeTo {
        let segs: Vec<&str> = from.split("::").collect();

        match segs[0].trim() {
            "Hello" => Type::Hello,
            "Left" => {
                if let Ok(id) = u32::from_str(segs[1].trim()) {
                    Type::Left(id)
                } else {
                    Type::Dropped
                }
            }
            "Moved" => {
                match (
                    u32::from_str(segs[1]),
                    u32::from_str(segs[2]),
                    u32::from_str(segs[3]),
                    u32::from_str(segs[4]),
                ) {
                    (Ok(id), Ok(x), Ok(y), Ok(d)) => Type::Moved(
                        id,
                        Transform {
                            position: (x, y),
                            direction: Direction::from_u32(d),
                        },
                    ),
                    _ => Type::Dropped,
                }
            }
            _ => Type::Dropped,
        }
    }
}
