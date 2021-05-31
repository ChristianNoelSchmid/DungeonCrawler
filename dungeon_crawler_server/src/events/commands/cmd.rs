use std::{
    num::ParseIntError,
    str::{FromStr, Split},
};

use crossbeam::channel::Sender;
use simple_serializer::Serialize;
use udp_server::packets::SendPacket;

use super::{combat::CombatCommand, status::StatusCommand, sync::SyncCommand};

pub trait Cmd<T> {
    fn process_client_cmd(cmd: T, s_to_state: Sender<Command>);
    fn process_state_msg(msg: T, s_to_dgm: Sender<SendPacket>);
}

pub enum Command {
    Sync(SyncCommand),
    Combat(CombatCommand),
    Status(StatusCommand),
    Abort,
}

impl FromStr for Command {
    type Err = ParseCmdErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = CmdArgs::from(s);
        match args.next()? {
            "Sync" => Ok(Command::Sync(SyncCommand::from_str(args.join_rest()?)?)),
            "Combat" => Ok(Command::Combat(CombatCommand::from_str(args.join_rest()?)?)),
            "Status" => Ok(Command::Status(StatusCommand::from_str(args.join_rest()?)?)),
            _ => Err(ParseCmdErr),
        }
    }
}

impl Serialize for Command {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        match self {
            Command::Sync(sync) => format!("Sync::{}", sync.serialize()),
            Command::Combat(combat) => format!("Combat::{}", combat.serialize()),
            Command::Status(status) => format!("Status::{}", status.serialize()),
            Command::Abort => "Abort::".to_string(),
        }
    }
}

pub struct ParseCmdErr;
impl From<ParseIntError> for ParseCmdErr {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

/// A wrapper for a `&str` split iterator.
/// Only two methods are provided for this struct:
/// `from`, which converts the &str into a `CmdArgs`
/// and `next`, which returns either the next arg, or
/// a `ParseCmdErr`. `next` removes a good deal of boilerplate
/// code from the `FromStr` implementations of different commands
pub struct CmdArgs<'a> {
    original: &'a str,
    args: Split<'a, &'a str>,
}
impl<'a> CmdArgs<'a> {
    pub fn from(data: &'a str) -> Self {
        Self {
            original: data,
            args: data.split("::"),
        }
    }
    pub fn next(&mut self) -> Result<&'a str, ParseCmdErr> {
        self.args.next().ok_or(ParseCmdErr)
    }
    pub fn join_rest(&mut self) -> Result<&'a str, ParseCmdErr> {
        Ok(self.original.split_once("::").ok_or(ParseCmdErr)?.1)
    }
}
