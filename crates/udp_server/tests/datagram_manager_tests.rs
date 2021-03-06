//! Tests for Udp Datagram Manager
//!
//! Christian Schmid - June 2021
//! CS510 - Rust Programming

#[cfg(test)]
mod datagram_handler_tests {

    use std::{net::SocketAddr, str::FromStr, thread, time::Duration};
    use udp_server::{
        manager::DatagramManager,
        packets::{ReceivePacket, SendPacket},
    };

    // Generates two DatagramManagers to test with
    fn gen_handlers(port1: u32, port2: u32) -> (DatagramManager, DatagramManager) {
        return (
            DatagramManager::new(port1).unwrap(),
            DatagramManager::new(port2).unwrap(),
        );
    }

    ///
    /// Tests simple communication between
    /// two DatagramManagers.
    ///
    #[test]
    fn test_send_recieve() {
        let (h1, h2) = gen_handlers(2000, 2001);

        // Retrieve the senders and receivers from the Managers
        let (s1, r1) = h1.get_sender_receiver();
        let (s2, r2) = h2.get_sender_receiver();

        // Send the first packet
        s1.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2001").unwrap()],
            is_rel: true,
            msg: "Hello!".to_string(),
        })
        .unwrap();

        // Ensure it was received by the other manager
        if let ReceivePacket::ClientMessage(addr, msg) = r2.recv().unwrap() {
            assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2000").unwrap());
            assert_eq!(msg, "Hello!");
        } else {
            panic!("Recieved ClientDropped message");
        }

        // Send the second packet
        s2.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2000").unwrap()],
            is_rel: true,
            msg: "Hi there!".to_string(),
        })
        .unwrap();

        // Ensure it was received by the other manager
        if let ReceivePacket::ClientMessage(addr, msg) = r1.recv().unwrap() {
            assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2001").unwrap());
            assert_eq!(msg, "Hi there!");
        } else {
            panic!("Recieved ClientDropped message");
        }
    }

    /// Sends 100 datagrams between two managers,
    /// ensuring that each one is successfully recieved,
    /// in order.
    #[test]
    fn test_bulk_send() {
        let (h1, h2) = gen_handlers(2004, 2005);

        let (s1, r1) = h1.get_sender_receiver();
        let (s2, r2) = h2.get_sender_receiver();

        // Send 50 from the first manager
        thread::spawn(move || {
            for _ in 0..50 {
                s1.send(SendPacket {
                    addrs: vec![SocketAddr::from_str("127.0.0.1:2005").unwrap()],
                    is_rel: true,
                    msg: "Hello!".to_string(),
                })
                .unwrap();
            }
        })
        .join()
        .unwrap();

        // Send 50 from the second manager
        thread::spawn(move || {
            for _ in 0..50 {
                s2.send(SendPacket {
                    addrs: vec![SocketAddr::from_str("127.0.0.1:2004").unwrap()],
                    is_rel: true,
                    msg: "Hello!".to_string(),
                })
                .unwrap();
            }
        })
        .join()
        .unwrap();

        thread::spawn(move || {
            for _ in 0..50 {
                // This will panic if a message is not
                // sent successfully
                r1.recv().unwrap();
            }
        })
        .join()
        .unwrap();

        thread::spawn(move || {
            for _ in 0..50 {
                // This will panic if a message is not
                // sent succesfully
                r2.recv().unwrap();
            }
        })
        .join()
        .unwrap();
    }

    ///
    /// Tests that a DatagramHandler continues to send reliable datagrams
    /// until receiving confirmation from the recipient. This also
    /// ensures that reliable messages are sent in order, by time sent.
    ///
    #[test]
    fn test_reliable_datagram() {
        let (h1, h2) = gen_handlers(2002, 2003);

        let (_, r1) = h1.get_sender_receiver();
        let (s2, _) = h2.get_sender_receiver();

        // Begin a thread that will send packets and sleep.
        // Altogether 25 packets are sent over 2.5 seconds
        std::thread::spawn(move || {
            for i in 0..25 {
                s2.send(SendPacket {
                    addrs: vec![SocketAddr::from_str("127.0.0.1:2002").unwrap()],
                    is_rel: true,
                    msg: i.to_string(),
                })
                .unwrap();
                std::thread::sleep(Duration::from_millis(100));
            }
        });

        // Sleep main thread for 1.25 seconds.
        std::thread::sleep(Duration::from_secs_f32(1.25));

        // Ensure that, even with the sleep, all reliable messages
        // are sent in order.
        for i in 0..25 {
            assert_eq!(
                r1.recv().unwrap(),
                ReceivePacket::ClientMessage(
                    SocketAddr::from_str("127.0.0.1:2003").unwrap(),
                    i.to_string()
                )
            );
        }
    }

    /// Tests that a client is successfully dropped when the timeout is reached
    #[test]
    fn test_drop_status() {
        let (h1, h2) = gen_handlers(2006, 2007);
        let (s1, _) = h1.get_sender_receiver();
        let (_, r2) = h2.get_sender_receiver();

        // Send a datagram
        s1.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2007").unwrap()],
            is_rel: true,
            msg: "hello!".to_string(),
        })
        .unwrap();
        r2.recv().unwrap();

        // Sleep until timeout
        std::thread::sleep(Duration::from_secs_f32(5.5));

        // Assert that the manager which received the message has informed the
        // program that a client has been dropped.
        assert_eq!(
            r2.recv().unwrap(),
            ReceivePacket::DroppedClient(SocketAddr::from_str("127.0.0.1:2006").unwrap())
        );
    }
}
