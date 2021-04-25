use std::str::FromStr;

pub enum Type {
    Hello,
    Joined(u32),
    Welcome,
    Left(u32),
    Drop,
}

impl Type {
    pub fn to_string(self) -> String {
        match self {
            Type::Hello => "Hello".to_string(),
            Type::Joined(id) => format!("Joined::{}", id),
            Type::Welcome => "Welcome".to_string(),
            Type::Left(id) => format!("Left::{}", id),
            Type::Drop => "Drop".to_string(),
        }
    }
    pub fn from_str(from: &str) -> Type {
        let segs: Vec<&str> = from.split("::").collect();

        match segs[0].trim() {
            "Hello" => Type::Hello,
            "Joined" => {
                if let Ok(id) = u32::from_str(segs[1].trim()) {
                    Type::Joined(id)
                } else {
                    Type::Drop
                }
            }
            "Welcome" => Type::Welcome,
            "Left" => {
                if let Ok(id) = u32::from_str(segs[1].trim()) {
                    Type::Left(id)
                } else {
                    Type::Drop
                }
            }
            _ => Type::Drop,
        }
    }
}
