use std::str::FromStr;

use dungeon_generator::inst::Dungeon;

#[derive(Debug)]
pub enum Type<'a> {
    Hello,
    Joined(u32),
    Welcome(u32, &'a Dungeon),
    Moved(u32, (u32, u32)),
    Left(u32),
    Dropped,
}

impl<'a> Type<'a> {
    pub fn to_string(self) -> String {
        match self {
            Type::Hello => "Hello".to_string(),
            Type::Joined(id) => format!("Joined::{}", id),
            Type::Welcome(id, dun) => format!("Welcome::{}::{}", id, build_dun_string(dun)),
            Type::Moved(id, (x, y)) => format!("Moved::{}::{}::{}", id, x, y),
            Type::Left(id) => format!("Left::{}", id),
            Type::Dropped => "Drop".to_string(),
        }
    }
    pub fn from_str(from: &str) -> Type {
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

fn build_dun_string(dun: &Dungeon) -> String {
    let mut path_str = String::new();

    path_str.push_str(&dun.paths().len().to_string());

    for path in dun.paths() {
        path_str.push_str("::");
        path_str.push_str(&path.0.to_string());
        path_str.push_str("::");
        path_str.push_str(&path.1.to_string());
    }

    path_str.push_str("::");
    path_str.push_str(&dun.entrance.0.to_string());
    path_str.push_str("::");
    path_str.push_str(&dun.entrance.1.to_string());

    path_str.push_str("::");
    path_str.push_str(&dun.exit.0.to_string());
    path_str.push_str("::");
    path_str.push_str(&dun.exit.1.to_string());

    path_str
}
