use rpg_server::datagram::{
    handler::DatagramHandler, 
    enums::SendTo
};

fn main() {
    let (handler, rx) = DatagramHandler::new(5000).unwrap();
    let tx = handler.get_sender();

    println!("UDP server started...");
    loop {
        match rx.recv() {
            Ok((addr, msg)) => {
                println!("Recieved string: \"{}\"", msg);
                println!("Returning to sender reversed...");

                tx.send((SendTo::One(addr), msg.chars().rev().collect::<String>()))
                    .unwrap();
            }
            _ => {}
        }
    }
}
