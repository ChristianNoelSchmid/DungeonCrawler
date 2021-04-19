use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    time::{Duration, Instant},
};

use super::{enums::RelResult, types::DatagramType};

pub struct AckResolver {
    pub addr: SocketAddr,
    pub index: u64,
    pub msg: String,
    pub start_time: Instant,
}

pub struct AckHandler {
    next_to: HashMap<SocketAddr, u64>,
    next_from: HashMap<SocketAddr, u64>,
    resolvers: HashMap<SocketAddr, VecDeque<AckResolver>>,
    timeout: Duration,
}

impl AckHandler {
    pub fn new() -> Self {
        AckHandler {
            next_to: HashMap::new(),
            next_from: HashMap::new(),
            resolvers: HashMap::new(),
            timeout: Duration::from_millis(500),
        }
    }

    pub fn accept_ack(&mut self, addr: SocketAddr, index: u64) -> Result<(), &str> {
        if self.next_from.contains_key(&addr) {
            let next_index = self.next_from[&addr];

            return if next_index < index {
                Ok(())
            } else if next_index == index {
                let resolver = self.resolvers.get_mut(&addr).unwrap().pop_back().unwrap();

                // Set the new timeout as the average between the
                // current timeout and time the RTT took for the
                // Ack datagram
                let lifespan = Instant::now() - resolver.start_time;
                self.timeout += lifespan;
                self.timeout /= 2;

                // Increment the expected ack
                self.next_from.insert(addr, index + 1);
                Ok(())
            } else {
                Err("Ack received gt current index.")
            };
        }
        Err("SocketAddr does not exist.")
    }

    ///
    /// Creates a new reliable datagram resolver that the AckHandler stores,
    /// retrieving the `addr` of the client, and the intended `msg`.
    /// Returns a `u64` representing the ack index of the reliable datagram
    /// being delivered.
    ///
    pub fn create_rel_resolver(&mut self, addr: SocketAddr, msg: String) -> u64 {
        // The index of the next reliable datagram being sent be addr
        let next_to_index;

        // Check if a reliable datagram has already been sent to this client,
        // and if so, grab the next index. Otherwise, add the client to next_to,
        // and create a new resolver list
        if !self.next_to.contains_key(&addr) {
            next_to_index = 0;
            self.next_to.insert(addr, 1);
            self.next_from.insert(addr, 0);
            self.resolvers.insert(addr, VecDeque::new());
        } else {
            next_to_index = self.next_to[&addr];
            self.next_to.insert(addr, next_to_index + 1);
        }

        let resolver = AckResolver {
            addr,
            msg,
            index: next_to_index,
            start_time: Instant::now(),
        };

        self.resolvers.get_mut(&addr).unwrap().insert(0, resolver);

        next_to_index
    }

    ///
    /// Checks an incoming reliable datagram from `addr` to see if it's has
    /// the `index_from` expected from the resolver's cache. An out of order
    /// index means that the reliable datagram is either a re-sent message already
    /// accepted by the server, or one that has come before a prior datagram.
    ///
    pub fn check_rel(&mut self, addr: SocketAddr, index_from: u64) -> RelResult {
        // Check if the client's addr is in the cache
        let index = self.next_from.get(&addr);

        match index {
            // If it's not in the cache, the reliable message should
            // be 0 (initial contact). If it's not, ask the client
            // to resend. Otherwise establish the new client and sent an Ack.
            None => {
                return if index_from == 0 {
                    self.next_from.insert(addr, 1);
                    RelResult::NewRel
                }
                else {
                    RelResult::NeedsRes
                }
            }
            // If there already is a ack index in the cache, see if
            // it matches the index received
            Some(index) => {
                let index = *index;
                // If it does, send an Ack back to the client
                // and increment the index in the cache.
                return if index == index_from {
                    self.next_from.insert(addr, index + 1);
                    RelResult::NewRel
                // If it's too low, this is a re-sent datagram which
                // has already been processed. Simply resend an Ack
                } else if index > index_from {
                    RelResult::RepeatedRel
                } else {
                    // Otherwise, it's too high: request the client to 
                    // resend its reliable messages stored in its own resolver
                    RelResult::NeedsRes
                }
            }
        }
    }
    pub fn retrieve_timeouts(&mut self) -> Vec<&mut AckResolver> {
        let mut resolvers = Vec::new();
        for list in self.resolvers.values_mut() {
            if let Some(resolver) = list.back_mut() {
                if Instant::now() - resolver.start_time > self.timeout {
                    resolver.start_time = Instant::now();
                    resolvers.push(resolver);
                }
            }
        }
        resolvers
    }
}
