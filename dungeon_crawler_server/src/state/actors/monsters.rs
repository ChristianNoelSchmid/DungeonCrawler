use std::rc::Rc;

use simple_serializer::Serialize;
use state::transforms::transform;

use crate::{
    astar::find_shortest_path,
    state::{
        self,
        stats::{Attributes, Stats},
        traits::{Combater, Directed, Positioned, TargetTranslator, Translater, AI},
        transforms::{positioner::WorldTransformer, transform::Transform, vec2::Vec2},
    },
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
    pub id: u32,
    pub name: &'static str,
    pub spawn_chance: u32,
}

#[derive(Clone)]
pub struct MonsterInstance {
    pub template: &'static Monster,
    pub instance_id: u32,

    transformer: Rc<WorldTransformer>,

    pub stats: Stats,
    pub path: Vec<Vec2>,
}

impl MonsterInstance {
    pub fn new(
        template: &'static Monster,
        instance_id: u32,
        transformer: Rc<WorldTransformer>,
    ) -> Self {
        Self {
            template,
            instance_id,

            transformer,
            stats: template.stats.clone(),
            path: Vec::new(),
        }
    }
    fn transform(&self) -> &Transform {
        self.transformer.transform(self.instance_id).unwrap()
    }
    fn transformer(&mut self) -> &mut WorldTransformer {
        Rc::get_mut(&mut self.transformer).unwrap()
    }
}

impl Positioned for MonsterInstance {
    fn pos(&self) -> Vec2 {
        self.transform().position
    }
}

impl Directed for MonsterInstance {
    fn dir(&self) -> transform::Direction {
        self.transform().direction
    }

    fn face_dir(&mut self, dir: transform::Direction) {
        let id = self.instance_id;
        self.transformer().change_dir(id, dir);
    }
}

impl Translater for MonsterInstance {
    fn move_pos(&mut self, new_pos: Vec2) -> bool {
        let id = self.instance_id;
        self.transformer().move_pos(id, new_pos)
    }

    fn change_trans(&mut self, new_t: Transform) -> bool {
        let id = self.instance_id;
        self.transformer().from_transform(id, new_t).is_some()
    }

    fn spot_within(&self, range: u32) -> Option<&Vec2> {
        self.transformer.open_spot_within(self.pos(), range)
    }
}

impl AI for MonsterInstance {}

impl TargetTranslator for MonsterInstance {
    fn target(&self) -> Option<&Vec2> {
        self.path.first()
    }

    fn next_to_target(&self) -> bool {
        return if let Some(target) = self.target() {
            Vec2::distance(self.pos(), *target) <= 1.0
        } else {
            false
        };
    }

    fn move_next(&mut self) {
        if let Some(point) = self.path.pop() {
            if !self.move_pos(point) {
                self.path = find_shortest_path(
                    &self.transformer,
                    self.pos(),
                    *self.path.first().unwrap_or(&point),
                )
            }
        }
    }

    fn set_target(&mut self, target: Vec2) {
        self.path = find_shortest_path(&self.transformer, self.pos(), target);
    }
}

impl Combater for MonsterInstance {
    fn attk(&self) {}
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
            self.template.id,
            self.instance_id,
            self.transform().serialize()
        )
    }
}
