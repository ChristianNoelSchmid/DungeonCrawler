use simple_serializer::Serialize;

use super::transform::{Transform};

///
/// Represents a particular monster type
/// in the game. The reason `name` is static
/// is to communicate that there are templates,
/// rather than the Monster objects themselves.
///
#[derive(Copy, Clone, Debug)]
pub struct Monster {
    pub template_id: u32,
    pub name: &'static str,
    pub spawn_chance: u32,
    pub range: u32,
    pub damage: u32,
}

#[derive(Clone, Debug)]
pub struct MonsterInstance {
    pub template: &'static Monster,
    pub instance_id: u32,
    pub transform: Transform,
    pub path: Vec<(i32, i32)>,
}

impl Serialize for MonsterInstance {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        // Serializing `MonsterInstance` only requires
        // the template id from `Monster`. The client
        // will have the rest of the information to
        // generate the `MonsterInstance` from it's
        // associated template id.
        format!(
            "{}::{}::{}",
            self.template.template_id,
            self.instance_id,
            self.transform.serialize(),
        )
    }
}
