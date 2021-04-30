use simple_serializer::Serialize;

///
/// Represents a particular monster type
/// in the game. The reason `name` is static
/// is to communicate that there are templates,
/// rather than the Monster objects themselves.
///
struct Monster {
    id: u32,
    name: &'static str,
    spawn_chance: u32,
    range: u32,
    damage: u32,
}

impl Serialize for Monster {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!(
            "{}::{}::{}::{}", 
            self.name, 
            self.spawn_chance, 
            self.range, 
            self.damage
        )
    } 
}