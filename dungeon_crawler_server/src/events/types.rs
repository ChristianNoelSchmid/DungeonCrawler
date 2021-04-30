use std::str::FromStr;

use dungeon_generator::inst::Dungeon;
use simple_serializer::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Type<'a> {
    Hello,
    Joined(u32),
    Welcome(u32, &'a Dungeon),
    Moved(u32, (u32, u32)),
    Left(u32),
    Dropped,
}

impl<'a> Serialize for Type<'a> {
    type SerializeTo = String;
    fn serialize(&self) -> String {
        match self {
            Type::Hello => "Hello".to_string(),
            Type::Joined(id) => format!("Joined::{}", id),
            Type::Welcome(id, dun) => format!("Welcome::{}::{}", id, dun.serialize()),
            Type::Moved(id, (x, y)) => format!("Moved::{}::{}::{}", id, x, y),
            Type::Left(id) => format!("Left::{}", id),
            Type::Dropped => "Drop".to_string(),
        }
    }
}

impl<'a> Deserialize for Type<'a> {

    type DeserializeTo = Type<'a>;

    fn deserialize(from: &str) -> Type<'a> {
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
                if let Ok(id) = u32::from_str(segs[1].trim()) {
                    if let Ok(x) = u32::from_str(segs[2]) {
                        if let Ok(y) = u32::from_str(segs[3]) {
                            return Type::Moved(id, (x, y));
                        }
                    }
                }
                Type::Dropped
            }
            _ => Type::Dropped,
        }
    }
}