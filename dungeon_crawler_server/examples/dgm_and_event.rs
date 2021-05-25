use dungeon_crawler_server::events::manager::EventManager;
use udp_server::manager::DatagramManager;

fn main() -> Result<(), std::io::Error> {
    let dgm_h = DatagramManager::new(2000)?;
    let (s, r) = dgm_h.get_sender_receiver();

    let mut evt_h = EventManager::new(r, s);
    evt_h.start();
}
