use rpg_server::datagram:: {
    handler::DatagramHandler,
    enums::SendTo
};
use rpg_server::dungeon::inst::Dungeon;

fn main() -> std::io::Result<()> {
    let handler = DatagramHandler::new(5000)?;
    let (s, r) = handler.get_sender_receiver();

    println!("UDP server started...");
    loop {
        match r.recv() {
            Ok((addr, msg)) => {
                println!("Recieved string from {:?}: \"{}\"", addr, msg.trim_end_matches("\n"));
                println!("Sending dungeon...");

                s.send((SendTo::One(addr), false, format!("{:?}", Dungeon::new(20, 20)))).unwrap();
            }
            _ => {}
        }
    }
}
