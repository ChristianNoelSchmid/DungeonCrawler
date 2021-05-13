use simple_serializer::Serialize;

use crate::state::{
    stats::{Attributes, Stats},
    traits::{Combater, Identified, Translator, AI},
    transforms::vec2::Vec2,
};

///
/// Represents a particular monster type
/// in the game. The reason `name` is static
/// is to communicate that there are templates,
/// rather than the Monster objects themselves.
///
#[derive(Clone)]
pub struct Monster {
    pub stats: Stats,
    pub attrs: Attributes,
    pub sight_range: u32,

    pub id: u32,
    pub name: &'static str,
    pub spawn_chance: u32,
}

#[derive(Clone)]
pub struct MonsterInstance {
    pub template: &'static Monster,
    pub instance_id: u32,
    pub stats: Stats,
    pub path: Vec<Vec2>,
    pub in_combat_with: Option<u32>,
}

impl MonsterInstance {
    pub fn new(template: &'static Monster, instance_id: u32) -> Self {
        Self {
            template,
            instance_id,
            stats: template.stats.clone(),
            path: Vec::new(),
            in_combat_with: None,
        }
    }
}

impl Identified for MonsterInstance {
    fn id(&self) -> u32 {
        self.instance_id
    }
}

impl AI for MonsterInstance {}

impl Translator for MonsterInstance {
    fn target(&self) -> Option<&Vec2> {
        self.path.first()
    }
    fn set_path(&mut self, path: Vec<Vec2>) {
        self.path = path;
    }

    fn next_step(&mut self) -> Option<Vec2> {
        self.path.pop()
    }
}

impl Combater for MonsterInstance {
    fn start_combat_with(&mut self, id: u32) {
        self.in_combat_with = Some(id)
    }
    fn in_combat_with(&self) -> Option<u32> {
        self.in_combat_with
    }
    fn sight_range(&self) -> u32 {
        self.template.sight_range
    }
}

impl Serialize for MonsterInstance {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        // Serializing `MonsterInstance` only requires
        // the template id from `Monster`. The client
        // will have the rest of the information to
        // generate the `MonsterInstance` from it's
        // associated template id.
        format!("{}::{}", self.template.id, self.instance_id,)
    }
}
