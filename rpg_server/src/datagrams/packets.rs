use std::net::SocketAddr;

pub struct SendPacket {
    pub addrs: Vec<SocketAddr>,
    pub is_rel: bool,
    pub msg: String,
}

pub struct ReceivePacket {
    pub addr: SocketAddr,
    pub msg: String,
}
