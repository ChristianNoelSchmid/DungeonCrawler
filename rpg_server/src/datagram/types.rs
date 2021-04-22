use std::str::FromStr;
///
/// All datagram types that can be sent
/// and/or recieved from the server
///
#[derive(Debug)]
pub enum Type {
    Unrel(String),    // unreliable datagram (does not need ack)
    Rel(u64, String), // reliable datagram (requires sending an ack)
    Ack(u64),         // ack datagram (acknowledges a rel datagram has been recieved)
    Res,              // resend datagram (the recipient needs all rel datagrams resent)
    Dis,  // a datagram representing a client that the server has dropped their connection
    Drop, // a datagram that had some kind of parsing error, and is simply dropped
}

impl Type {
    pub fn to_buffer(self) -> Vec<u8> {
        match self {
            Self::Unrel(data) => data.to_string(),
            Self::Rel(ack_index, data) => format!("REL::{}::{}", ack_index, data),
            Self::Ack(ack_index) => format!("ACK::{}", ack_index),
            Self::Res => "RES".to_string(),
            Self::Dis => "DIS".to_string(),
            Self::Drop => "DROP".to_string(),
        }
        .into_bytes() // Convert the resulting string into bytes
    }

    pub fn from_str(s: &str) -> Type {
        let segs = s.split("::").collect::<Vec<&str>>();

        match segs[0].trim() {
            "REL" => {
                if let Ok(index) = u64::from_str(segs[1].trim()) {
                    return Type::Rel(index, segs[2].trim().to_string());
                }
                Type::Drop
            }
            "ACK" => {
                if let Ok(index) = u64::from_str(segs[1].trim()) {
                    return Type::Ack(index);
                }
                Type::Drop
            }
            "RES" => Type::Res,
            _ => Type::Unrel(segs.join("").to_string()),
        }
    }
}
