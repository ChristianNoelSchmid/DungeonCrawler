use udp_server::handler::DatagramHandler;
use rpg_server::events::handler::EventHandler;

fn main() -> Result<(), std::io::Error> {
    let dgm_h = DatagramHandler::new(2000)?;
    let (s, r) = dgm_h.get_sender_receiver();

    let mut evt_h = EventHandler::new(r, s);
    evt_h.start();
}
