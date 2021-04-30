use std::{collections::HashMap, net::SocketAddr};

use simple_serializer::{Deserialize, Serialize};
use udp_server::packets::{PacketReceiver, PacketSender, ReceivePacket, SendPacket};
use dungeon_generator::inst::Dungeon;

use super::types::Type;


pub struct EventHandler {
    r_from_client: PacketReceiver,
    s_to_clients: PacketSender,
    dun: Dungeon,

    addrs: HashMap<SocketAddr, u32>,
    id_next: u32,
}

impl EventHandler {
    ///
    /// Creates a new EventHandler, and receives a DatagramHandler's
    /// client Receiver `r_from_client` and Sender `s_to_clients`.
    /// This enables concurrent communication with the DatagramHandler.
    ///
    pub fn new(r_from_client: PacketReceiver, s_to_clients: PacketSender) -> Self {
        EventHandler {
            r_from_client,
            s_to_clients,
            dun: Dungeon::new(100, 100),

            addrs: HashMap::new(),
            id_next: 0,
        }
    }

    ///
    /// Starts the EventHandler: begins an infinite
    /// loop to begin receiving packets from the DatagramHandler
    ///
    pub fn start(&mut self) -> ! {
        loop {
            if let Ok(packet) = self.r_from_client.recv() {
                let snd_packets = self.parse_packet(packet);
                for packet in snd_packets {
                    self.s_to_clients.send(packet).unwrap();
                }
            }
        }
    }

    ///
    /// Parses a Datagram ReceivePacket `packet`, determining what needs
    /// to be accomplished on the server state, and what messages need to
    /// be sent back to the clients.
    ///
    fn parse_packet(&mut self, packet: ReceivePacket) -> Vec<SendPacket> {
        return match packet {
            ReceivePacket::DroppedClient(addr) => self.drop_client(addr),
            ReceivePacket::ClientMessage(addr, msg) => self.parse_client_msg((addr, msg)),
        };
    }

    ///
    /// Drops the supplied client `addr` from the EventHandler's
    /// system. Generally called via client request, or when
    /// the server's connection with the client has timed out
    ///
    fn drop_client(&mut self, addr: SocketAddr) -> Vec<SendPacket> {
        let mut snd_packets = Vec::new();
        if let Some(id) = self.addrs.remove(&addr) {
            snd_packets.push(SendPacket {
                addrs: self.all_addrs(),
                is_rel: true,
                msg: Type::Left(id).serialize(),
            });
        }
        snd_packets
    }

    ///
    /// Parses the `msg` received from the DatagramHandler from client `addr`,
    /// determing the appropriate course of action, and performing it.
    ///
    fn parse_client_msg(&mut self, (addr, msg): (SocketAddr, String)) -> Vec<SendPacket> {
        let mut snd_packets = Vec::new();

        // Parse the msg into an appropriate event
        let event = Type::deserialize(&msg);

        match event {
            Type::Hello => {
                snd_packets.push(SendPacket {
                    addrs: vec![addr],
                    is_rel: true,
                    msg: Type::Welcome(self.id_next, &self.dun).serialize(),
                });
                snd_packets.push(SendPacket {
                    addrs: self.all_addrs(),
                    is_rel: true,
                    msg: Type::Joined(self.id_next).serialize(),
                });
                self.addrs.insert(addr, self.id_next);
                self.id_next += 1;
            }
            Type::Moved(id, (x, y)) => {
                if self.addrs.contains_key(&addr) {
                    snd_packets.push(SendPacket {
                        addrs: self.all_addrs_but(addr),
                        is_rel: false,
                        msg: Type::Moved(id, (x, y)).serialize(),
                    });
                }
            }
            _ => {}
        };
        snd_packets
    }

    fn all_addrs(&self) -> Vec<SocketAddr> {
        self.addrs.clone().into_iter().map(|(k, _)| k).collect()
    }
    fn all_addrs_but(&self, addr: SocketAddr) -> Vec<SocketAddr> {
        self.addrs
            .clone()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .map(|(k, _)| k)
            .collect()
    }
}
