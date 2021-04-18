use std::net::SocketAddr;

pub enum SendTo {
    One(SocketAddr),
    AllBut(SocketAddr),
    All,
}

#[derive(PartialEq, Eq)]
pub enum HandlerState {
    Listening,
    Stopped,
    Dropped,
}