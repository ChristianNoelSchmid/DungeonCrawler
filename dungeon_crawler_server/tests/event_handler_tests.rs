#[cfg(test)]
mod event_handler_tests {
    use dungeon_crawler_server::events::manager::EventManager;
    use udp_server::manager::DatagramManager;

    #[test]
    fn test_send_receive() {
        todo!();
        let mut dgm_h1 = DatagramManager::new(6000).unwrap();
        let (s, r) = dgm_h1.get_sender_receiver();
        let mut evn_h1 = EventManager::new(r, s);
        evn_h1.start();

        let dgm_h2 = DatagramManager::new(6001);
    }
}
