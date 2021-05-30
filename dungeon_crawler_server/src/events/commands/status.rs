use std::str::FromStr;

use simple_serializer::Serialize;

use super::cmd::{CmdArgs, ParseCmdErr};

pub enum StatusCommand {
    // Server to Client
    Dead(u32),    // informs clients that a Player has died       (id)
    Escaped(u32), // informs clients that a Player has escaped    (id)
}

impl Serialize for StatusCommand {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        match self {
            StatusCommand::Dead(id) => format!("Dead::{}", id),
            StatusCommand::Escaped(id) => format!("Escaped::{}", id),
        }
    }
}

impl FromStr for StatusCommand {
    type Err = ParseCmdErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = CmdArgs::from(s);
        match args.next()? {
            "Dead" => Ok(StatusCommand::Dead(args.next()?.parse()?)),
            "Escaped" => Ok(StatusCommand::Escaped(args.next()?.parse()?)),
            _ => Err(ParseCmdErr),
        }
    }
}
