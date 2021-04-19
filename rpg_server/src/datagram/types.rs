use std::str::FromStr;
///
/// All datagram types that can be sent
/// and recieved from the server
///
pub enum DatagramType {
    Unrel(String),    // unreliable datagram (does not need ack)
    Rel(u64, String), // reliable datagram (requires sending an ack)
    Ack(u64),         // ack datagram (acknowledges a rel datagram has been recieved)
    Res,              // resend datagram (the recipient needs all rel datagrams resent)
}

impl DatagramType {
    pub fn to_buffer(self) -> Vec<u8> {
        match self {
            Self::Unrel(data) => data.to_string(),
            Self::Rel(ack_index, data) => format!("REL::{}::{}", ack_index, data),
            Self::Ack(ack_index) => format!("ACK::{}", ack_index),
            Self::Res => "RES".to_string(),
        }
        .into_bytes()
    }
}

impl FromStr for DatagramType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segs = s.split("::").collect::<Vec<&str>>();

        match segs[0].trim() {
            "REL" => Ok(DatagramType::Rel(
                u64::from_str(segs[1].trim()).expect("Not u64"),
                segs[2].to_string(),
            )),
            "ACK" => Ok(DatagramType::Ack(
                u64::from_str(segs[1].trim()).expect("Not u64"),
            )),
            "RES" => Ok(DatagramType::Res),
            _ => Ok(DatagramType::Unrel(segs.join("").to_string())),
        }
    }
}
