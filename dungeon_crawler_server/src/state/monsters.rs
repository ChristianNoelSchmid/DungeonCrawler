use std::time::{Duration, Instant};

use simple_serializer::Serialize;

use crate::state::{
    stats::{Attributes, Stats},
    traits::{Follower, Identified, Translator, AI},
    transforms::vec2::Vec2,
};

use super::traits::{AttackStatus, Combater};

///
/// Represents a particular monster type
/// in the game. All `MonsterInstance`s are built
/// on these templates.
///
#[derive(Clone)]
pub struct Monster {
    // base Stats
    pub stats: Stats,
    // base Attributes
    pub attrs: Attributes,

    // The distance the Monster can see
    pub sight_range: u32,

    // The Monster template's id
    pub id: u32,
    // The name of the Monster
    pub name: &'static str,
    // The chance the Monster has to spawn.
    // The higher the value (in comparison to the
    // other Monsters' spawn_chance's) the higher
    // the chance of choosing this Monster
    pub spawn_chance: u32,
}

///
/// An instance of a `Monster` template.
/// implementing all the traits necessary
/// to perform AI.
///
#[derive(Clone)]
pub struct MonsterInstance {
    pub template: &'static Monster,

    // Identified
    pub instance_id: u32,

    // Translator
    path: Vec<Vec2>,
    charge_step: Option<Instant>,

    // Follower
    follow_target: Option<u32>,
    last_sighting: Instant,

    // Combator
    charge_attk: Option<Instant>,
    charging: bool,
}

impl MonsterInstance {
    /// Create a new `MonsterInstance` using the given `template`,
    /// and assigning it a unique `instance_id`
    pub fn new(template: &'static Monster, instance_id: u32) -> Self {
        Self {
            template,
            instance_id,
            path: Vec::new(),
            follow_target: None,
            last_sighting: Instant::now(),

            charge_step: None,
            charge_attk: None,
            charging: false,
        }
    }
}

impl Identified for MonsterInstance {
    fn id(&self) -> u32 {
        self.instance_id
    }
}

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
    fn charge_step(&mut self) -> bool {
        if let Some(step) = self.charge_step {
            if Instant::now() - step > Duration::from_millis(200) {
                self.charge_step = None;
                return true;
            }
        } else {
            self.charge_step = Some(Instant::now());
        }
        false
    }
}

impl Follower for MonsterInstance {
    fn follow_target(&self) -> Option<u32> {
        self.follow_target
    }
    fn start_following(&mut self, id: u32) {
        self.follow_target = Some(id)
    }
    fn stop_following(&mut self) {
        self.follow_target = None;
    }
    fn sight_range(&self) -> u32 {
        self.template.sight_range
    }
    fn last_sighting(&self) -> Instant {
        self.last_sighting
    }
    fn reset_last_sighting(&mut self) {
        self.last_sighting = Instant::now();
    }
}

impl Combater for MonsterInstance {
    fn charge_attk(&mut self) -> AttackStatus {
        if let Some(attk) = self.charge_attk {
            match Instant::now() - attk {
                n if n > Duration::from_millis(750) => {
                    self.charge_attk = None;
                    return AttackStatus::Charged;
                }
                n if n > Duration::from_millis(250) => {
                    if !self.charging {
                        self.charging = true;
                        return AttackStatus::BegunCharging;
                    } else {
                        return AttackStatus::Charging;
                    }
                }
                _ => self.charging = false,
            }
        } else {
            self.charge_attk = Some(Instant::now());
            self.charging = false;
        }
        AttackStatus::NotReady
    }
    fn reset_attk(&mut self) {
        self.charge_attk = None;
        self.charging = false;
    }
}

impl AI for MonsterInstance {}

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
