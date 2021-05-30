use std::{net::SocketAddr, str::FromStr};

use simple_serializer::Serialize;

use super::cmd::{CmdArgs, ParseCmdErr};
use crate::state::transforms::{
    transform::{Direction, Transform},
    vec2::Vec2,
};

pub enum SyncCommand {
    // Both
    Moved(u32, Transform), // informs server / clients of moved entity          (id, transform)
    PlayerLeft(u32),       // informs server / clients that a Player has left   (id)

    // Client to Server
    Hello(String), // a client joining the game                               (name)

    // Server to Client
    Welcome(u32, String), // info. the server relays to the client for syncing       (id, dun_data)
    NewPlayer(u32, String, Vec2), // informs clients of a new Player,                (id, name, pos)
    NewMonster(u32, u32, Vec2), // informs clients of a new Monster                  (temp_id, inst_id, pos)
    Reconnect,                  // requests that the clients reconnect - new StateManager
    DungeonComplete,            // informs clients that the Dungeon has been completed

    // Server Synchronization
    CreatePlayer(SocketAddr, String),
    WelcomePlayer(SocketAddr, u32, String, Vec2),
}

impl Serialize for SyncCommand {
    type SerializeTo = String;
    fn serialize(&self) -> String {
        match self {
            SyncCommand::Moved(id, transform) => {
                format!("Moved::{}::{}", id, transform.serialize())
            }
            SyncCommand::PlayerLeft(id) => format!("PlayerLeft::{}", id),
            SyncCommand::Welcome(id, dun) => format!("Welcome::{}::{}", id, dun),
            SyncCommand::NewPlayer(id, name, pos) => {
                format!("NewPlayer::{}::{}::{}", id, name, pos.serialize())
            }
            SyncCommand::NewMonster(t_id, i_id, pos) => {
                format!("NewMonster::{}::{}::{}", t_id, i_id, pos.serialize())
            }
            SyncCommand::Reconnect => "Reconnect::".to_string(),
            SyncCommand::DungeonComplete => "DungeonComplete::".to_string(),

            // For the SyncCommands that aren't ever sent to clients,
            // return an empty String
            _ => "".to_string()
        }
    }
}

impl FromStr for SyncCommand {
    type Err = ParseCmdErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segs = CmdArgs::from(s);
        match segs.next()? {
            "Moved" => Ok(SyncCommand::Moved(
                segs.next()?.parse()?,
                Transform::with_values(
                    Vec2(segs.next()?.parse()?, segs.next()?.parse()?),
                    Direction::from_u32(segs.next()?.parse()?),
                ),
            )),
            "PlayerLeft" => Ok(SyncCommand::PlayerLeft(segs.next()?.parse()?)),
            "Hello" => Ok(SyncCommand::Hello(segs.next()?.to_string())),
            _ => Err(ParseCmdErr),
        }
    }
}
