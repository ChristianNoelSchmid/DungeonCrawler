use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::mpsc::Sender,
    time::{Instant},
};

use super::enums::SendTo;

pub struct AckResolver {
    addr: SocketAddr,
    index: u64,
    msg: String,
    start_time: Instant,
}

pub struct AckResolverHandler {
    next_to: HashMap<SocketAddr, u64>,
    next_from: HashMap<SocketAddr, u64>,
    resolvers: HashMap<SocketAddr, Vec<AckResolver>>,
    tx: Sender<(SendTo, String)>,
}

impl AckResolverHandler {
    pub fn new(tx: Sender<(SendTo, String)>) -> Self {
        AckResolverHandler {
            next_to: HashMap::new(),
            next_from: HashMap::new(),
            resolvers: HashMap::new(),
            tx,
        }
    }

    fn accept_ack(&mut self, addr: SocketAddr, index: u64) {
        if self.next_from.contains_key(&addr) {
            let resolver = self.resolvers.get_mut(&addr).unwrap().pop().unwrap();
            let lifespan = Instant::now() - resolver.start_time;
        } else if index == 0 {
            self.next_from.insert(addr, 0);
        }
    }

    fn await_ack(&mut self, addr: SocketAddr, msg: String) {
        let next_to_index;

        if !self.next_to.contains_key(&addr) {
            next_to_index = 0;
            self.next_to.insert(addr, next_to_index + 1);
            self.resolvers.insert(addr, Vec::new());
        } else {
            next_to_index = self.next_to[&addr];
            *(self.next_to.get_mut(&addr).unwrap()) += 1;
        }

        let resolver = AckResolver {
            addr,
            msg,
            index: next_to_index,
            start_time: Instant::now(),
        };

        self.resolvers.get_mut(&addr).unwrap().push(resolver);
    }

    fn check_rel(&mut self, addr: SocketAddr, index_from: u64) {
        if !self.next_from.contains_key(&addr) {
            if index_from != 0 {}
        }
    }
}
