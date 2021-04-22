#[cfg(test)]
mod datagram_handler_tests {

    use rpg_server::datagram::{
        handler::DatagramHandler,
        packets::{ReceivePacket, SendPacket},
    };
    use std::{net::SocketAddr, str::FromStr, time::Duration};

    fn gen_handlers(port1: u32, port2: u32) -> (DatagramHandler, DatagramHandler) {
        return (
            DatagramHandler::new(port1).unwrap(),
            DatagramHandler::new(port2).unwrap(),
        );
    }

    ///
    /// Tests simple (unreliable) communication between
    /// two DatagramHandlers.
    ///
    #[test]
    fn test_send_recieve() {
        let (h1, h2) = gen_handlers(2000, 2001);

        let (s1, r1) = h1.get_sender_receiver();
        let (s2, r2) = h2.get_sender_receiver();

        s1.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2001").unwrap()],
            is_rel: true,
            msg: "Hello!".to_string(),
        })
        .unwrap();

        let ReceivePacket { addr, msg } = r2.recv().unwrap();
        assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2000").unwrap());
        assert_eq!(msg, "Hello!");

        s2.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2000").unwrap()],
            is_rel: true,
            msg: "Hi there!".to_string(),
        })
        .unwrap();

        let ReceivePacket { addr, msg } = r1.recv().unwrap();
        assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2001").unwrap());
        assert_eq!(msg, "Hi there!");
    }

    ///
    /// Tests that a DatagramHandler continues to send reliable datagrams
    /// until receiving confirmation from the recipient. This also
    /// ensures that reliable messages are sent in order, by time sent.
    ///
    #[test]
    fn test_reliable_datagram() {
        let (mut h1, h2) = gen_handlers(2002, 2003);

        let (_, r1) = h1.get_sender_receiver();
        let (s2, _) = h2.get_sender_receiver();

        // Put the recipient to sleep, so it won't received the
        // messages immediately
        h1.set_listening(false);

        s2.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2002").unwrap()],
            is_rel: true,
            msg: "Hi!".to_string(),
        })
        .unwrap();

        s2.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2002").unwrap()],
            is_rel: true,
            msg: "there!".to_string(),
        })
        .unwrap();

        std::thread::sleep(Duration::from_millis(1000));

        // Send one more datagram, to ensure it isn't the first
        // one parsed.
        s2.send(SendPacket {
            addrs: vec![SocketAddr::from_str("127.0.0.1:2002").unwrap()],
            is_rel: true,
            msg: "Once more!".to_string(),
        })
        .unwrap();

        // Wake up the recipient, and attempt to get
        // the messages
        h1.set_listening(true);

        // The messages received should be received in this order.
        let ReceivePacket { msg, .. } = r1.recv().unwrap();
        assert_eq!(msg, "Hi!");

        let ReceivePacket { msg, .. } = r1.recv().unwrap();
        assert_eq!(msg, "there!");

        let ReceivePacket { msg, .. } = r1.recv().unwrap();
        assert_eq!(msg, "Once more!");
    }
}
