use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    time::{Duration, Instant},
};

use super::enums::RelResult;

pub struct AckResolver {
    pub addr: SocketAddr,
    pub index: u64,
    pub msg: String,

    start_time: Instant,
    last_update_time: Instant,
}

/// 
/// Manages a cache of reliable datagrams which have
/// yet to be acknowledged by the recipient.
/// Enables reliable, in-order UDP transmission.
///
pub struct AckResolverManager {
    // A map of the next rel indices the server
    // will send to a target recipient.
    next_to: HashMap<SocketAddr, u64>, 
    // A map of the next rel indices the server
    // is expecting from target recipients.
    next_from: HashMap<SocketAddr, u64>,
    // The cache of reliable datagrams. Added to when
    // a new rel is sent and removed from after the client
    // has acknowledged the rel datagram.
    resolvers: HashMap<SocketAddr, VecDeque<AckResolver>>,
    // A map of Durations representing the average amount
    // of RTT time between the server and a unique address.
    timeouts: HashMap<SocketAddr, Duration>,
}

impl AckResolverManager {
    /// Creates a new `AckResolverManager` with default values.
    pub fn new() -> Self {
        Self {
            next_to: HashMap::new(),
            next_from: HashMap::new(),
            resolvers: HashMap::new(),
            timeouts: HashMap::new(),
        }
    }

    /// Removes the specified reliable datagram with the given
    /// `addr` and `index`. Called after the DatagramHandler
    /// accepts a client ACK datagram.
    pub fn accept_ack(&mut self, addr: SocketAddr, index: u64) {
        let mut pop_back = false; // avoids mutability ownership issues
        // Retrieve the AckResolver
        if let Some(resolver_list) = self.resolvers.get_mut(&addr) {
            if let Some(resolver) = resolver_list.back() {
                if resolver.index == index {
                    // Set the new timeout as the average between the
                    // current timeout and time the RTT took for the
                    // Ack datagram
                    let lifespan = Instant::now() - resolver.start_time;
                    let timeout_secs = (self.timeouts[&addr] + lifespan) / 2;

                    self.timeouts.insert(addr, timeout_secs);
                    pop_back = true;
                }
            }
            // If successful, remove the AckResolver from the list
            if pop_back {
                resolver_list.pop_back();
            }
        }
    }

    ///
    /// Creates a new reliable datagram resolver that the AckHandler stores,
    /// retrieving the `addr` of the client, and the intended `msg`.
    /// Returns a `u64` representing the ack index of the reliable datagram
    /// being delivered.
    ///
    pub fn create_rel_resolver(&mut self, addr: SocketAddr, msg: String) -> u64 {
        // Check if a reliable datagram has already been sent to this client,
        // and if so, grab the next index. Otherwise, add the client to next_to and
        // next_from, and create a new resolver list
        let next_to = self.next_to.entry(addr).or_insert(0);
        *next_to += 1;

        let resolver = AckResolver {
            addr,
            msg,
            index: *next_to - 1,
            start_time: Instant::now(),
            last_update_time: Instant::now(),
        };

        self.resolvers
            .entry(addr)
            .or_insert_with(VecDeque::new)
            .insert(0, resolver);
        self.timeouts
            .entry(addr)
            .or_insert_with(|| Duration::from_millis(500));

        *next_to - 1
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
                // If the client isn't in the server's cache, but the
                // ack index is 0, this represents a new connection.
                // Add the client addr to the cache, and return NewRel
                if index_from == 0 {
                    self.next_from.insert(addr, 1);
                    RelResult::NewRel
                } else {
                    // If the client isn't in the server's cache, and it's
                    // ack index is above 0, the server assumes that the
                    // client was previously connected, but has been dropped.
                    // Inform the client of such!
                    RelResult::ClientDropped
                }
            }
            // If there already is a ack index in the cache, see if
            // it matches the index received
            Some(index) => {
                let index = *index;
                // If it does, send an Ack back to the client
                // and increment the index in the cache.
                match index {
                    i if i == index_from => {
                        self.next_from.insert(addr, index + 1);
                        RelResult::NewRel
                    }
                    // If it's too low, this is a re-sent datagram which
                    // has already been processed. Simply resend an Ack
                    i if i > index_from => RelResult::RepeatedRel,

                    // Otherwise, it's too high: request the client to
                    // resend its reliable messages stored in its own resolver
                    _ => RelResult::NeedsRes,
                }
            }
        }
    }

    /// Removes an `addr` from the AckResolverManager.
    /// Any unresolved messages associated with the client
    /// are dropped.
    pub fn remove_client(&mut self, addr: SocketAddr) {
        self.resolvers.remove(&addr);
        self.next_from.remove(&addr);
        self.next_to.remove(&addr);
    }

    /// Triggered when the DatagramManager receives a RES datagram.
    /// Requests that the AckResolverManager retrieve all reliable
    /// datagrams applied to `addr`.
    pub fn resend_to(&mut self, addr: SocketAddr) -> Vec<&AckResolver> {
        let mut resolvers = Vec::new();
        if let Some(list) = self.resolvers.get_mut(&addr) {
            for resolver in list.iter().rev() {
                resolvers.push(resolver);
            }
        }
        resolvers
    }

    /// Retrieves all timeout reliable datagrams.
    /// The DatagramManager calls this to resend reliable
    /// datagrams that have yet to be acknowledged by the client.
    pub fn retrieve_timeouts(&mut self) -> Vec<&mut AckResolver> {
        let mut resolvers = Vec::new();
        for list in self.resolvers.values_mut() {
            if let Some(resolver) = list.back_mut() {
                if Instant::now() - resolver.last_update_time > self.timeouts[&resolver.addr] {
                    resolver.last_update_time = Instant::now();
                    resolvers.push(resolver);
                }
            }
        }
        resolvers
    }
}
