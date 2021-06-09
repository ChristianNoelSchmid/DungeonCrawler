//! Datagram Packet Types for Udp Datagram Manager
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

use std::str::FromStr;

use simple_serializer::{Deserialize, Serialize};
///
/// All datagram types that can be sent
/// and/or recieved from the server
///
#[derive(Debug)]
pub enum Type {
    /// unreliable datagram (does not need ack)
    Unrel(String),
    /// reliable datagram (requires sending an ack)
    Rel(u64, String),
    /// ack datagram (acknowledges a rel datagram has been recieved)
    /// with it's associated index
    Ack(u64),
    /// resend datagram (the recipient needs all rel datagrams resent)
    Res,
    /// a datagram representing to the server that a client is still connected
    /// If enough time passes where the client doesn't send this, or any other,
    /// datagram, the server will drop it.
    Ping,
    /// a datagram that had some kind of parsing error, or the server informing a
    /// client that it has been dropped
    Drop,
}

impl Serialize for Type {
    type SerializeTo = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Unrel(data) => format!("UNR::{}", data.to_string()),
            Self::Rel(ack_index, data) => format!("REL::{}::{}", ack_index, data),
            Self::Ack(ack_index) => format!("ACK::{}", ack_index),
            Self::Res => "RES".to_string(),
            Self::Ping => "PNG".to_string(),
            Self::Drop => "DRP".to_string(),
        }
        .into_bytes() // Convert the resulting string into bytes
    }
}

impl Deserialize for Type {
    type DeserializeTo = Type;
    fn deserialize(from: &str) -> Type {
        let segs = from.split("::").collect::<Vec<&str>>();
        match segs[0].trim() {
            "UNR" => Type::Unrel(segs[1..].join("::")),
            "REL" => {
                return if let Ok(index) = u64::from_str(segs[1].trim()) {
                    Type::Rel(index, segs[2..].join("::"))
                } else {
                    Type::Drop
                }
            }
            "ACK" => {
                return if let Ok(index) = u64::from_str(segs[1].trim()) {
                    Type::Ack(index)
                } else {
                    Type::Drop
                }
            }
            "RES" => Type::Res,
            "PNG" => Type::Ping,
            _ => Type::Drop,
        }
    }
}
