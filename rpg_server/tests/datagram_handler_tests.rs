use std::{net::SocketAddr, str::FromStr};

use rpg_server::datagram::{
    handler::DatagramHandler, 
    enums::SendTo
};

#[test]
fn test_send_recieve() {
    let (handler1, rx1) = DatagramHandler::new(2000).unwrap();
    let (handler2, rx2) = DatagramHandler::new(2001).unwrap();

    let (tx1, tx2) = (handler1.get_sender(), handler2.get_sender());

    tx1.send((
        SendTo::One(SocketAddr::from_str("127.0.0.1:2001").unwrap()),
        "Hello!".to_string(),
    ))
    .unwrap();

    let (addr, msg) = rx2.recv().unwrap();
    assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2000").unwrap());
    assert_eq!(msg, "Hello!");

    tx2.send((
        SendTo::One(SocketAddr::from_str("127.0.0.1:2000").unwrap()),
        "Hi there!".to_string(),
    ))
    .unwrap();

    let (addr, msg) = rx1.recv().unwrap();
    assert_eq!(addr, SocketAddr::from_str("127.0.0.1:2001").unwrap());
    assert_eq!(msg, "Hi there!");
}
