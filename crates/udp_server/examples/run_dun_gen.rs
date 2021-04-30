use udp_server::{
    handler::DatagramHandler,
    packets::{ReceivePacket, SendPacket},
};

use dungeon_generator::inst::Dungeon;

const PORT: u32 = 5000;

fn main() -> std::io::Result<()> {
    let handler = DatagramHandler::new(PORT)?;
    let (s, r) = handler.get_sender_receiver();
    let mut addrs = Vec::new();

    println!("\nUDP server started (127.0.0.1:{})", PORT);
    println!("Send a message to this server and receive a datagram in response!");

    loop {
        if let Ok(ReceivePacket::ClientMessage(addr, msg)) = r.recv() {
            println!(
                "Recieved string from {:?}: \"{}\"",
                addr,
                msg.trim_end_matches("\n")
            );
            if !addrs.contains(&addr) {
                addrs.push(addr);
            }
        }

        let new_dun = format!("{:?}", Dungeon::new(30, 30));
        s.send(SendPacket {
            addrs: addrs.clone(),
            is_rel: false,
            msg: new_dun,
        })
        .unwrap();
    }
}
