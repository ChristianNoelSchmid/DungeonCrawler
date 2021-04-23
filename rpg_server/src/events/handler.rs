use std::{ net::SocketAddr, collections::HashSet };

use crate::datagrams::packets::{ReceivePacket, SendPacket};
use crossbeam::channel::{Receiver, Sender};

use super::types::Type;

pub struct EventHandler {
    r_from_client: Receiver<ReceivePacket>,
    s_to_clients: Sender<SendPacket>,

    addrs: HashSet<SocketAddr>,
    id_next: u32
}

impl EventHandler {
    ///
    /// Creates a new EventHandler, and receives a DatagramHandler's
    /// client Receiver `r_from_client` and Sender `s_to_clients`. 
    /// This enables concurrent communication with the DatagramHandler.
    ///
    pub fn new(r_from_client: Receiver<ReceivePacket>, s_to_clients: Sender<SendPacket>) -> Self {
        EventHandler {
            r_from_client,
            s_to_clients,

            addrs: HashSet::new(),
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

        // Destruct the ReceivePacket into its constituent parts
        let ReceivePacket { addr, msg } = packet;
        let mut snd_packets = Vec::new();

        // Parse the msg into an appropriate event
        let event = Type::from_str(&msg);

        match event {
            Type::Hello => {
                snd_packets.push(SendPacket {
                    addrs: vec![addr],
                    is_rel: true,
                    msg: Type::Welcome.to_string()
                });
                snd_packets.push(SendPacket {
                    addrs: self.addrs.clone().into_iter().collect(),
                    is_rel: true,
                    msg: Type::Joined(self.id_next).to_string()
                });
                self.addrs.insert(addr);
                self.id_next += 1;
            },
            Type::Left => {

            },
            _ => {} 
        };

        snd_packets
    }
}
