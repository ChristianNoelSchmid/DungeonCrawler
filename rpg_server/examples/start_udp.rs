use std::collections::HashSet;

use rpg_server::datagram::{
    handler::DatagramHandler,
    packets::{ReceivePacket, SendPacket},
};

use rpg_server::dungeon::inst::Dungeon;

fn main() -> std::io::Result<()> {
    let handler = DatagramHandler::new(5000)?;
    let (s, r) = handler.get_sender_receiver();
    let mut addrs = HashSet::new();

    println!("UDP server started...");
    loop {
        if let Ok(ReceivePacket { addr, msg }) = r.try_recv() {
            println!(
                "Recieved string from {:?}: \"{}\"",
                addr,
                msg.trim_end_matches("\n")
            );
            addrs.insert(addr);
        }

        let new_dun = format!("{:?}", Dungeon::new(3, 3));
        s.send(SendPacket {
            addrs: addrs.clone().into_iter().collect(),
            is_rel: false,
            msg: new_dun,
        })
        .unwrap();
    }
}
