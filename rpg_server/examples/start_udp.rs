use rpg_server::datagram::{handler::DatagramHandler};

fn main() {
    let handler = DatagramHandler::new(5000).unwrap();
    let (_, r) = handler.get_sender_receiver();

    println!("UDP server started...");
    loop {
        match r.recv() {
            Ok((addr, msg)) => {
                println!("Recieved string from {:?}: \"{}\"", addr, msg);
            }
            _ => {}
        }
    }
}
