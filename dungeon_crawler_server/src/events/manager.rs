use std::{collections::HashMap, net::SocketAddr, time::Duration};

use crossbeam::channel::{Receiver, Sender};
use dungeon_generator::inst::Dungeon;
use simple_serializer::{Deserialize, Serialize};
use udp_server::packets::{PacketReceiver, PacketSender, ReceivePacket, SendPacket};

use crate::{
    events::types::Type,
    state::{
        manager::StateManager,
        snapshot::StateSnapshot,
        types::{RequestType, ResponseType},
    },
};

pub struct EventManager {
    state_manager: StateManager,

    r_from_client: PacketReceiver,
    s_to_clients: PacketSender,
    s_to_state: Sender<RequestType>,
    r_from_state: Receiver<ResponseType>,

    addrs: HashMap<SocketAddr, u32>,
    id_next: u32,
}

impl EventManager {
    ///
    /// Creates a new EventHandler, and receives a DatagramHandler's
    /// client Receiver `r_from_client` and Sender `s_to_clients`.
    /// This enables concurrent communication with the DatagramHandler.
    ///
    pub fn new(r_from_client: PacketReceiver, s_to_clients: PacketSender) -> Self {
        let dun = Dungeon::new(75, 75);
        let state_manager = StateManager::new(dun);
        let (s_to_state, r_from_state) = state_manager.get_sender_receiver();

        for i in 0..10 {
            s_to_state.send(RequestType::SpawnMonster(i)).unwrap();
        }

        EventManager {
            state_manager,
            r_from_client,
            s_to_clients,
            s_to_state,
            r_from_state,
            addrs: HashMap::new(),
            id_next: 10,
        }
    }

    ///
    /// Starts the EventHandler: begins an infinite
    /// loop to begin receiving packets from the DatagramHandler
    ///
    pub fn start(&mut self) -> ! {
        loop {
            if let Ok(packet) = self.r_from_client.try_recv() {
                let snd_packets = self.parse_client_packet(packet);
                for packet in snd_packets {
                    self.s_to_clients.send(packet).unwrap();
                }
            }
            if let Ok(response) = self.r_from_state.try_recv() {
                self.parse_state_response(response);
            }
        }
    }

    /// Parses a Datagram ReceivePacket `packet`, determining what needs
    /// to be accomplished on the server state, and what messages need to
    /// be sent back to the clients.
    fn parse_client_packet(&mut self, packet: ReceivePacket) -> Vec<SendPacket> {
        match packet {
            ReceivePacket::DroppedClient(addr) => self.drop_client(addr),
            ReceivePacket::ClientMessage(addr, msg) => self.parse_client_msg((addr, msg)),
        }
    }

    /// Drops the supplied client `addr` from the EventHandler's
    /// system. Generally called via client request, or when
    /// the server's connection with the client has timed out
    fn drop_client(&mut self, addr: SocketAddr) -> Vec<SendPacket> {
        let mut snd_packets = Vec::new();
        if let Some(id) = self.addrs.remove(&addr) {
            snd_packets.push(SendPacket {
                addrs: self.all_addrs(),
                is_rel: true,
                msg: Type::PlayerLeft(id).serialize(),
            });

            self.s_to_state.send(RequestType::DropPlayer(id)).unwrap();
        }
        snd_packets
    }

    ///
    /// Parses the `msg` received from the DatagramHandler from client `addr`,
    /// determing the appropriate course of action, and performing it.
    ///
    fn parse_client_msg(&mut self, (addr, msg): (SocketAddr, String)) -> Vec<SendPacket> {
        let mut snd_packets = Vec::new();
        println!("{}", msg);

        // Parse the msg into an appropriate event
        let event = Type::deserialize(&msg);

        match event {
            Type::Hello(name) => {
                self.s_to_state
                    .send(RequestType::NewPlayer(addr, self.id_next, name))
                    .unwrap();
                self.addrs.insert(addr, self.id_next);
                self.id_next += 1;
            }
            Type::Moved(id, transform) => {
                // If Moved, update in state and send to other clients
                if self.addrs.contains_key(&addr) {
                    self.s_to_state
                        .send(RequestType::PlayerMoved(id, transform))
                        .unwrap();
                    snd_packets.push(SendPacket {
                        addrs: self.all_addrs_but(addr),
                        is_rel: false,
                        msg: Type::Moved(id, transform).serialize(),
                    });
                }
            }
            _ => {}
        };
        snd_packets
    }

    fn parse_state_response(&mut self, response: ResponseType) {
        match response {
            ResponseType::MonsterMoved(id, transform) => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: false,
                        msg: Type::Moved(id, transform).serialize(),
                    })
                    .unwrap();
            }
            ResponseType::StateSnapshot(snapshot) => {
                let snd_msg_packets = self.prepare_welcome_packet(snapshot);
                for packet in snd_msg_packets.into_iter() {
                    self.s_to_clients.send(packet).unwrap();
                }
            }
            ResponseType::Hit(att_id, def_id, cur_health) => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: false,
                        msg: Type::Hit(att_id, def_id, cur_health).serialize(),
                    })
                    .unwrap();
            }
            ResponseType::Miss(att_id, def_id) => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: false,
                        msg: Type::Miss(att_id, def_id).serialize(),
                    })
                    .unwrap();
            }
            ResponseType::Dead(id) => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: true,
                        msg: Type::Dead(id).serialize(),
                    })
                    .unwrap();
            }
            ResponseType::Escaped(id) => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: true,
                        msg: Type::Escaped(id).serialize(),
                    })
                    .unwrap();
            }
            ResponseType::DungeonComplete => {
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: true,
                        msg: Type::DungeonComplete.serialize(),
                    })
                    .unwrap();

                std::thread::sleep(Duration::from_secs(5));

                self.state_manager = StateManager::new(Dungeon::new(75, 75));
                let (s, r) = self.state_manager.get_sender_receiver();
                self.s_to_state = s;
                self.r_from_state = r;
                self.s_to_clients
                    .send(SendPacket {
                        addrs: self.all_addrs(),
                        is_rel: true,
                        msg: Type::Reconnect.serialize(),
                    })
                    .unwrap();

                for i in self.id_next..self.id_next + 10 {
                    self.s_to_state.send(RequestType::SpawnMonster(i)).unwrap();
                }

                self.id_next += 10;
            }
            _ => {}
        }
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

    /// A collections of UDP packets which give a joining `addr`
    /// all information relating to the current server state.
    /// Also prepares a message to all other clients informing them
    /// of the newcomer.
    fn prepare_welcome_packet(&mut self, snapshot: StateSnapshot) -> Vec<SendPacket> {
        let mut snd_packets = Vec::new();

        // Send all MonsterInstance information to the client
        for monster in snapshot.monsters {
            snd_packets.push(SendPacket {
                addrs: vec![snapshot.addr_for],
                is_rel: true,
                msg: Type::NewMonster(monster.0, monster.1, monster.2).serialize(),
            });
        }

        for player in snapshot.other_players {
            snd_packets.push(SendPacket {
                addrs: vec![snapshot.addr_for],
                is_rel: true,
                msg: Type::NewPlayer(player.0, player.1, player.2).serialize(),
            });
        }

        for player_ts in snapshot.all_player_ts {
            snd_packets.push(SendPacket {
                addrs: vec![snapshot.addr_for],
                is_rel: true,
                msg: Type::Moved(player_ts.0, player_ts.1).serialize(),
            });
        }

        // Send to all connected clients the
        // new player info
        snd_packets.push(SendPacket {
            addrs: self.all_addrs_but(snapshot.addr_for),
            is_rel: true,
            msg: Type::NewPlayer(
                snapshot.new_player.0,
                snapshot.new_player.1,
                snapshot.new_player.2,
            )
            .serialize(),
        });
        // Send the Welcome packet to the incoming client,
        // which contains the dungeon information
        snd_packets.push(SendPacket {
            addrs: vec![snapshot.addr_for],
            is_rel: true,
            msg: Type::Welcome(snapshot.new_player.0, snapshot.dungeon.serialize()).serialize(),
        });

        snd_packets
    }
}
