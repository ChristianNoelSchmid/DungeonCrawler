use std::{net::SocketAddr, str::FromStr};

use rpg_server::datagram::{enums::SendTo, handler::DatagramHandler};

#[test]
fn test_send_recieve() {
    let handler1 = DatagramHandler::new(2000).unwrap();
    let handler2 = DatagramHandler::new(2001).unwrap();

    let (s1, r1) = handler1.get_sender_receiver();
    let (s2, r2) = handler2.get_sender_receiver();

    s1.send((
        SendTo::One(SocketAddr::from_str("127.0.0.1:2001").unwrap()),
        true,
        "Hello!".to_string(),
    ))
    .unwrap();

    let (addr, msg) = r2.recv().unwrap();
    assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2000").unwrap());
    assert_eq!(msg, "Hello!");

    s2.send((
        SendTo::One(SocketAddr::from_str("127.0.0.1:2000").unwrap()),
        true,
        "Hi there!".to_string(),
    ))
    .unwrap();

    let (addr, msg) = r1.recv().unwrap();
    assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2001").unwrap());
    assert_eq!(msg, "Hi there!");
}
