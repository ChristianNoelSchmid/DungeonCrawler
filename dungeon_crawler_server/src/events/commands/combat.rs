use simple_serializer::Serialize;
use std::str::FromStr;

use crate::{events::commands::cmd::ParseCmdErr, state::transforms::vec2::Vec2};

use super::cmd::CmdArgs;

pub enum CombatCommand {
    // Both
    AttackTowards(u32, Vec2), // informs clients that an entity is attacking towards (id, pos)

    // Server to Client
    Charging(u32),
    Hit(u32, u32, i32), // informs clients that a Player has been hit        (attId, defId, healthLeft)
    Miss(u32, u32),     // informs clients that a Player has been missed     (attId, defId)
}

impl Serialize for CombatCommand {
    type SerializeTo = String;

    fn serialize(&self) -> Self::SerializeTo {
        match self {
            CombatCommand::AttackTowards(id, pos) => {
                format!("AttackTowards::{}::{}", id, pos.serialize())
            }
            CombatCommand::Charging(id) => format!("Charging::{}", id),
            CombatCommand::Hit(attk_id, defd_id, health) => {
                format!("Hit::{}::{}::{}", attk_id, defd_id, health)
            }
            CombatCommand::Miss(attk_id, defd_id) => format!("Miss::{}::{}", attk_id, defd_id),
        }
    }
}

impl FromStr for CombatCommand {
    type Err = ParseCmdErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segs = CmdArgs::from(s);
        match segs.next()? {
            "AttackTowards" => Ok(CombatCommand::AttackTowards(
                segs.next()?.parse()?,
                Vec2(segs.next()?.parse()?, segs.next()?.parse()?),
            )),
            _ => Err(ParseCmdErr),
        }
    }
}
