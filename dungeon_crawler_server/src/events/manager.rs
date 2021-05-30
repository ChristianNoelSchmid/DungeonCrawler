use std::{collections::HashMap, net::SocketAddr, str::FromStr, time::Duration};

use crossbeam::channel::{Receiver, SendError, Sender};
use dungeon_generator::inst::Dungeon;
use simple_serializer::Serialize;
use udp_server::packets::{PacketReceiver, PacketSender, ReceivePacket, SendPacket};

use crate::{events::types::Type, state::manager::StateManager};

use super::commands::{cmd::Command, sync::SyncCommand};

pub enum SendTo {
    One(u32),
    AllBut(u32),
    All,
}

///
/// Handles receiving data from the DatagramManager, parsing the data,
/// converting the game state based on said data, and passing on the relevant
/// information to associated clients.
///
pub struct EventManager {
    // The state manager, which handles all the server's
    // world state
    state_manager: StateManager,

    // The PacketSender / Receiver associated with the
    // DatagramManager used to retrieve client packets.
    r_from_clients: PacketReceiver,
    s_to_clients: PacketSender,

    // The PacketSender / Receiver associated with the
    // StateManager's update thread.
    s_to_state: Sender<Command>,
    r_from_event: Receiver<Command>,

    r_from_state: Receiver<(Command, SendTo)>,

    // The currently connected addrs. Is added to when the
    // DatagramManager sends a packet from a new SocketAddr,
    // and removes when the DatagramManager times out a client.
    addrs: HashMap<SocketAddr, u32>,
}

impl EventManager {
    /// Creates a new EventHandler, and receives a DatagramHandler's
    /// client Receiver `r_from_client` and Sender `s_to_clients`.
    pub fn new(r_from_clients: PacketReceiver, s_to_clients: PacketSender) -> Self {
        let dun = Dungeon::new(75, 75);
        let (s_to_state, r_from_event) = crossbeam::channel::unbounded();

        let state_manager = StateManager::new(dun, 10, r_from_event.clone());
        let r_from_state = state_manager.get_receiver();

        EventManager {
            state_manager,

            s_to_clients,
            r_from_clients,

            s_to_state,
            r_from_event,

            r_from_state,

            addrs: HashMap::new(),
        }
    }

    /// Starts the EventHandler: begins an infinite
    /// loop to begin receiving packets from the DatagramHandler
    pub fn start(&mut self) -> Result<(), SendError<SendPacket>> {
        loop {
            if let Ok(packet) = self.r_from_clients.try_recv() {
                self.parse_client_packet(packet).unwrap();
            }
            if let Ok((cmd, send_to)) = self.r_from_state.try_recv() {
                self.parse_state_response(cmd, send_to)?;
            }
        }
    }

    /// Parses a Datagram ReceivePacket `packet`, determining what needs
    /// to be accomplished on the server state, and what messages need to
    /// be sent back to the clients.
    fn parse_client_packet(&mut self, packet: ReceivePacket) -> Result<(), SendError<Command>> {
        match packet {
            ReceivePacket::DroppedClient(addr) => self.drop_client(addr),
            ReceivePacket::ClientMessage(addr, msg) => self.parse_client_msg((addr, msg))?,
        }
        Ok(())
    }

    /// Drops the supplied client `addr` from the EventHandler's
    /// system. Generally called via client request, or when
    /// the server's connection with the client has timed out
    fn drop_client(&mut self, addr: SocketAddr) {
        if let Some(id) = self.addrs.remove(&addr) {
            self.s_to_state
                .send(Command::Sync(SyncCommand::PlayerLeft(id)))
                .unwrap();
        }
    }

    ///
    /// Parses the `msg` received from the DatagramHandler from client `addr`,
    /// determing the appropriate course of action, and performing it.
    ///
    fn parse_client_msg(&mut self, (addr, msg): (SocketAddr, String)) -> Result<(), SendError<Command>> {
        if let Ok(cmd) = Command::from_str(&msg) {
            self.s_to_state.send(match cmd {
                // If a `Hello` message was sent, CreatePlayer must
                // be called to ensure the new client is assigned an
                // Id. addr is given so the StateManager can send the
                // correct Id to the correct associate address.
                Command::Sync(SyncCommand::Hello(name)) => {
                    Command::Sync(SyncCommand::CreatePlayer(
                        addr,
                        name
                    ))
                },
                _ => cmd
            })?;
        }
        Ok(())
    }

    /// Parses responses sent by the `StateManager`, and sends the
    /// information to the appropriate clients.
    fn parse_state_response(
        &mut self,
        response: Command,
        send_to: SendTo,
    ) -> Result<(), SendError<SendPacket>> {
        // If the state registered that all Players are either dead or escaped,
        // reset the StateManager, creating a new dungeon.
        if let Command::Sync(SyncCommand::DungeonComplete) = response {
            self.s_to_clients.send(SendPacket {
                addrs: self.all_addrs(),
                is_rel: true,
                msg: Type::DungeonComplete.serialize(),
            })?;

            std::thread::sleep(Duration::from_secs(5));

            self.state_manager =
                StateManager::new(Dungeon::new(75, 75), 10, self.r_from_event.clone());
            let r = self.state_manager.get_receiver();
            self.r_from_state = r;
            self.s_to_clients.send(SendPacket {
                addrs: self.all_addrs(),
                is_rel: true,
                msg: Type::Reconnect.serialize(),
            })?;
        }
        Ok(())
    }

    /// Retrieve all `SocketAddr`s attached to the EventHandler
    fn all_addrs(&self) -> Vec<SocketAddr> {
        self.addrs.clone().into_iter().map(|(k, _)| k).collect()
    }
    /// Retrieve all `SocketAddr`'s attach to the EventHandler,
    /// exepct the `addr` provided.
    fn all_addrs_but(&self, addr: SocketAddr) -> Vec<SocketAddr> {
        self.addrs
            .clone()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .map(|(k, _)| k)
            .collect()
    }
}
