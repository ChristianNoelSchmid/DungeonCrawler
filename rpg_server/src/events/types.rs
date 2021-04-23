use std::str::FromStr;

pub enum Type {
    Hello,
    Joined(u32),
    Welcome,
    Left,
    Drop
}

impl Type {
    pub fn to_string(self) -> String {
        match self {
            Type::Hello => "Hello".to_string(),
            Type::Joined(id) => format!("Joined::{}", id),
            Type::Welcome => "Welcome".to_string(),
            Type::Left => "Left".to_string(),
            Type::Drop => "Drop".to_string()
        }
    }
    pub fn from_str(from: &str) -> Type {
        let segs: Vec<&str> = from.split("::").collect();
        
        match segs[0].trim() {
            "Hello" => Type::Hello,
            "Joined" => {
                if let Ok(id) = u32::from_str(segs[1]) {
                    Type::Joined(id)
                } else {
                    Type::Drop
                }
            },
            "Welcome" => Type::Welcome,
            "Left" => Type::Left,
            _ => Type::Drop,
        }
    }
}
