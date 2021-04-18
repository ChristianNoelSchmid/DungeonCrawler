use std::str::FromStr;
///
/// All datagram types that can be sent
/// and recieved from the server
///
pub enum DatagramType {
    Unrel(String), // unreliable datagram (does not need ack)
    Rel(String),   // reliable datagram (requires sending an ack)
    Ack(u64),      // ack datagram (acknowledges a rel datagram has been recieved)
    Res,           // resend datagram (the recipient needs all rel datagrams resent)
}

impl FromStr for DatagramType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("::");
        let (prefix, suffix) = (s.next().unwrap(), s.collect::<String>());

        match prefix {
            "REL" => Ok(DatagramType::Rel(suffix)),
            "ACK" => Ok(DatagramType::Ack(u64::from_str(suffix.as_str()).unwrap())),
            "RES" => Ok(DatagramType::Res),
            _ => Ok(DatagramType::Unrel(suffix)),
        }
    }
}
