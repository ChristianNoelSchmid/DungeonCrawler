use rpg_server::{datagrams::handler::DatagramHandler, events::handler::EventHandler};
use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let dgm_h = DatagramHandler::new(2000)?;
    let (s, r) = dgm_h.get_sender_receiver();

    let mut evt_h = EventHandler::new(r, s);
    evt_h.start();
}
