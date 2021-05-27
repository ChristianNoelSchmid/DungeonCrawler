use std::net::SocketAddr;

use crossbeam::channel::{Receiver, RecvError, SendError, Sender, TryRecvError};

///
/// A wrapper for a channel Sender
///
#[derive(Clone)]
pub struct PacketSender {
    s_to_clients: Sender<SendPacket>,
}

///
/// A wrapper for a channel Receiver
///
#[derive(Clone)]
pub struct PacketReceiver {
    r_from_clients: Receiver<ReceivePacket>,
}

impl PacketSender {
    /// Create a new PacketSender, with the supplied `s_to_clients` `Sender`
    pub fn new(s_to_clients: Sender<SendPacket>) -> Self {
        Self { s_to_clients }
    }
    /// Inform the `DatagramManager` that the specified `packet` needs
    /// to be sent
    pub fn send(&self, packet: SendPacket) -> Result<(), SendError<SendPacket>> {
        self.s_to_clients.send(packet)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientDroppedError;

impl PacketReceiver {
    /// Creates a new PacketReceiver, with the specified
    /// `Receiver` `r_from_clients`
    pub fn new(r_from_clients: Receiver<ReceivePacket>) -> Self {
        Self { r_from_clients }
    }
    /// Attempts to receive a package, blocking the
    /// current thread until successful
    pub fn recv(&self) -> Result<ReceivePacket, RecvError> {
        self.r_from_clients.recv()
    }
    /// Attempts to receive a package without blocking.
    pub fn try_recv(&self) -> Result<ReceivePacket, TryRecvError> {
        self.r_from_clients.try_recv()
    }
}

///
/// Contains all necessary data to send a datagram packet
/// to client(s)
///
pub struct SendPacket {
    pub addrs: Vec<SocketAddr>,
    pub is_rel: bool,
    pub msg: String,
}

///
/// Represents parsed datagram information that
/// a client has sent.
///
#[derive(Debug, PartialEq, Eq)]
pub enum ReceivePacket {
    ClientMessage(SocketAddr, String),
    DroppedClient(SocketAddr),
}
