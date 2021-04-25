use std::{collections::HashMap, net::SocketAddr};

use crate::datagrams::packets::{PacketReceiver, PacketSender, ReceivePacket, SendPacket};

use super::types::Type;

pub struct EventHandler {
    r_from_client: PacketReceiver,
    s_to_clients: PacketSender,

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
        }
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
                addrs: self.addrs(),
                is_rel: true,
                msg: Type::Left(id).to_string(),
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
        let event = Type::from_str(&msg);

        match event {
            Type::Hello => {
                snd_packets.push(SendPacket {
                    addrs: vec![addr],
                    is_rel: true,
                    msg: Type::Welcome.to_string(),
                });
                snd_packets.push(SendPacket {
                    addrs: self.addrs(),
                    is_rel: true,
                    msg: Type::Joined(self.id_next).to_string(),
                });
                self.addrs.insert(addr, self.id_next);
                self.id_next += 1;
            }
            _ => {}
        };
        snd_packets
    }

    fn addrs(&self) -> Vec<SocketAddr> {
        self.addrs.clone().into_iter().map(|(k, _)| k).collect()
    }
}
