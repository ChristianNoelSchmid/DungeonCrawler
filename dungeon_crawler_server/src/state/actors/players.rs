use std::rc::Rc;

use simple_serializer::Serialize;
use transforms::vec2;

use crate::state::{
    stats::{Attributes, Stats},
    traits::{Directed, Positioned, Translater},
    transforms::{self, positioner::WorldTransformer, transform::Transform},
};

///
/// Represents a Player in the state
///
#[derive(Debug, Clone)]
pub struct Player {
    stats: Stats,
    attrs: Attributes,
    transformer: Rc<WorldTransformer>,
    pub id: u32,
    pub name: String,
}

impl Player {
    pub fn new(id: u32, name: String, transformer: Rc<WorldTransformer>) -> Self {
        Self {
            stats: Stats::new(20, 10, 10),
            attrs: Attributes::new(4, 4, 4),
            transformer,
            id,
            name,
        }
    }
    fn transform(&self) -> &Transform {
        self.transformer.transform(self.id).unwrap()
    }
    fn transformer(&mut self) -> &mut WorldTransformer {
        Rc::get_mut(&mut self.transformer).unwrap()
    }
}

impl Positioned for Player {
    fn pos(&self) -> vec2::Vec2 {
        self.transform().position
    }
}

impl Directed for Player {
    fn dir(&self) -> transforms::transform::Direction {
        self.transform().direction
    }

    fn face_dir(&mut self, dir: transforms::transform::Direction) {
        let id = self.id;
        self.transformer().change_dir(id, dir);
    }
}

impl Translater for Player {
    fn move_pos(&mut self, new_pos: vec2::Vec2) -> bool {
        let id = self.id;
        self.transformer().move_pos(id, new_pos)
    }

    fn change_trans(&mut self, new_t: Transform) -> bool {
        let id = self.id;
        self.transformer().from_transform(id, new_t).is_some()
    }

    fn spot_within(&self, range: u32) -> Option<&vec2::Vec2> {
        self.transformer.open_spot_within(self.pos(), range)
    }
}

///
/// Serialization for the Player
/// (To String)
///
impl Serialize for Player {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        format!("{}", self.id)
    }
}
