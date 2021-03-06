//! UDP Tester - sending of UUIDs via 2 Udp sockets
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

use std::{net::SocketAddr, str::FromStr, time::Duration};

use udp_server::{
    manager::DatagramManager,
    packets::{ReceivePacket, SendPacket},
};

use uuid::Uuid;

const PRINT_UUIDS: bool = true;

fn main() -> std::io::Result<()> {
    let h1 = DatagramManager::new(5000)?;
    let h2 = DatagramManager::new(5001)?;

    let h3 = DatagramManager::new(5002)?;
    let addrs = vec![SocketAddr::from_str("127.0.0.1:5002").unwrap()];

    let (s1, _) = h1.get_sender_receiver();
    let (s2, _) = h2.get_sender_receiver();

    let (_, r3) = h3.get_sender_receiver();

    println!("\n\nSending as many UUID's per second as possible!");
    println!("(Two UDP senders, one receiver)...");
    if PRINT_UUIDS {
        println!("Warning! There may be a bottleneck on printing the UUIDs.");
    }
    std::thread::sleep(Duration::from_secs(6));

    std::thread::spawn(move || loop {
        let msg = format!("{}", Uuid::new_v4());

        s1.send(SendPacket {
            addrs: addrs.clone(),
            is_rel: false,
            msg,
        })
        .unwrap();

        let msg = format!("{}", Uuid::new_v4());

        s2.send(SendPacket {
            addrs: addrs.clone(),
            is_rel: false,
            msg,
        })
        .unwrap();
    });

    let mut count = 0;
    loop {
        if let ReceivePacket::ClientMessage(_, msg) = r3.recv().unwrap() {
            count += 1;

            if PRINT_UUIDS {
                println!("{} >> {}", msg, count);
            } else if count % 1000 == 0 {
                println!("{}", count);
            }
        }
    }
}
