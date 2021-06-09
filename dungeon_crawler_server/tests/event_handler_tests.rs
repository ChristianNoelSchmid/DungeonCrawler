//! State Manager tests
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

#[cfg(test)]
mod event_handler_tests {
    use std::{
        net::SocketAddr,
        str::FromStr,
        thread,
        time::{Duration, Instant},
    };

    use dungeon_crawler_server::events::{manager::EventManager, types::Type};
    use simple_serializer::Serialize;
    use udp_server::{
        manager::DatagramManager,
        packets::{ReceivePacket, SendPacket},
    };
    use ReceivePacket::ClientMessage;

    /// Generates a `DatagramManager` and `EventManager` on `port1` and `port2`.
    /// The `DatagramManager` is set up ready to send information to the newly
    /// created `EventManager` given the returned `SocketAddr`.
    fn gen_managers(port1: u32, port2: u32) -> (DatagramManager, SocketAddr) {
        let dgm = DatagramManager::new(port1).unwrap();

        thread::spawn(move || {
            let evt_man_dgm = DatagramManager::new(port2).unwrap();
            let (s, r) = evt_man_dgm.get_sender_receiver();
            let mut evt_man = EventManager::new(r, s);
            evt_man.start()
        });

        let evt_addr = SocketAddr::from_str(&format!("127.0.0.1:{}", port2)).unwrap();

        (dgm, evt_addr)
    }

    /// Tests that clients which send a `Hello` `EventType` receive
    /// a `Welcome` packet in return.
    #[test]
    fn test_new_player() {
        // Generate the managers
        let (dgm, evt_addr) = gen_managers(3000, 3001);
        let (s1, r1) = dgm.get_sender_receiver();
        let (s2, _) = DatagramManager::new(3004).unwrap().get_sender_receiver();

        s2.send(SendPacket {
            addrs: vec![evt_addr],
            is_rel: true,
            msg: Type::Hello("Sam".to_string()).serialize(),
        })
        .unwrap();
        thread::sleep(Duration::from_secs_f32(1.5));

        s1.send(SendPacket {
            addrs: vec![evt_addr],
            is_rel: true,
            msg: Type::Hello("Phil".to_string()).serialize(),
        })
        .unwrap();

        let now = Instant::now();
        assert!(loop {
            if Instant::now() - now > Duration::from_secs(3) {
                break false;
            } else if let Ok(ClientMessage(_, msg)) = r1.try_recv() {
                if msg.starts_with("Welcome::") {
                    break true;
                }
            }
        });
    }
}
