#[cfg(test)]
mod event_handler_tests {
    use rpg_server::datagrams::handler::DatagramHandler;
    use rpg_server::events::handler::EventHandler;

    #[test]
    fn test_send_receive() {
        todo!();
        let mut dgm_h1 = DatagramHandler::new(6000).unwrap();
        let (s, r) = dgm_h1.get_sender_receiver();
        let mut evn_h1 = EventHandler::new(r, s);
        evn_h1.start();

        let dgm_h2 = DatagramHandler::new(6001);
    }
}
