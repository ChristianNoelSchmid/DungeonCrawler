//! RPG Server - Datagram Handler
//!
//! Christian Schmid - April 2021
//! CS510 - Rust Programming

use super::{
    enums::{HandlerState, RelResult},
    packets::{
        PacketReceiver, PacketSender, ReceivePacket,
        ReceivePacket::{ClientMessage, DroppedClient},
        SendPacket,
    },
    resolver::AckHandler,
    types::Type,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use simple_serializer::{Deserialize, Serialize};

use std::{collections::{HashMap, HashSet}, net::{SocketAddr, UdpSocket}, sync::{Arc, Mutex}, thread, time::{Duration, Instant}};

const DEFAULT_DROP_TIME: Duration = Duration::from_secs(5);

///
/// A udp datagram handler, which recieves
/// the lowest-level byte data from incoming
/// clients.
///
/// The datagrams sent by clients are
/// parsed into their appropriate types, and handled
/// based on type (see datagram/types.rs)
///
pub struct DatagramHandler {
    // A PacketSender, which server threads can
    // use to send clients datagrams
    packet_sender: PacketSender,
    packet_receiver: PacketReceiver,

    // Two crossbeam Senders, which allows the DatagramHandler
    // thread to communicate to its constituent threads
    // the state of the handler (ie. listening, or not listening,
    // aborting)
    s_to_clients_state: Sender<HandlerState>,
    s_from_clients_state: Sender<HandlerState>,
}

impl DatagramHandler {
    ///
    /// Creates a new udp socket reciever / listener, on specified `port`.
    /// Begins listening for datagrams from clients.
    ///
    pub fn new(port: u32) -> std::io::Result<Self> {
        // Attempt to create the UdpSocket.
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;
        socket.set_nonblocking(true)?;

        // Convert the socket to a Arc Mutex, for concurrency
        let socket = Arc::new(Mutex::new(socket));

        let ack_resolver = Arc::new(Mutex::new(AckHandler::new()));

        // Create the channels which will handle synchronizing
        // state between handler threads
        let (s_to_clients_state, r_to_clients_state) = unbounded();
        let (s_from_clients_state, r_from_clients_state) = unbounded();

        // Begin the thread where the socket recieves datagrams from
        // clients, parses them, and passes relevant information,
        // to other threads in the server
        let r_from_clients =
            Self::receive_clients_loop(socket.clone(), ack_resolver.clone(), r_from_clients_state);

        // Begin the thread where the socket awaits other threads
        // in the server to send information to clients its
        // connected with
        let s_to_clients = Self::transmit_to_clients_loop(socket, ack_resolver, r_to_clients_state);

        let packet_sender = PacketSender::new(s_to_clients);
        let packet_receiver = PacketReceiver::new(r_from_clients);

        Ok(Self {
            packet_sender,
            packet_receiver,

            s_to_clients_state,
            s_from_clients_state,
        })
    }

    ///
    /// Clones a `Sender` and `Receiver`, to be used in other systems
    ///
    pub fn get_sender_receiver(&self) -> (PacketSender, PacketReceiver) {
        (self.packet_sender.clone(), self.packet_receiver.clone())
    }

    ///
    /// Starts or stops the `DatagramHandler`'s sending
    /// and receiving of data, determined by `is_listening`.
    ///
    pub fn set_listening(&mut self, is_listening: bool) {
        let state = if is_listening {
            HandlerState::Listening
        } else {
            HandlerState::Stopped
        };

        self.s_from_clients_state.send(state).unwrap();
        self.s_to_clients_state.send(state).unwrap();
    }

    ///
    /// Begins the receive loop for a concurrent `socket`, returning
    /// the `Reciever` which awaits messages from clients
    ///
    fn receive_clients_loop(
        socket: Arc<Mutex<UdpSocket>>,
        ack_resolver: Arc<Mutex<AckHandler>>,
        r_handler_state: Receiver<HandlerState>,
    ) -> Receiver<ReceivePacket> {
        // Create the Sender and Receiver
        let (s, r): (_, Receiver<ReceivePacket>) = unbounded();
        let mut state = HandlerState::Listening;
        let mut client_ping_times = HashMap::<SocketAddr, Instant>::new();
        let mut dropped_clients = HashSet::<SocketAddr>::new();

        // Spawn a new thread, and move the Sender.
        // The thread undergoes an infinite loop, awaiting
        // datagrams received by the socket
        std::thread::spawn(move || loop {
            state = r_handler_state.try_recv().unwrap_or(state);
            let mut buf = [0; 100];

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {}
            }

            // Retrieve a lock on the socket
            let socket = socket.lock().unwrap();
            // Retrieve a lock on the AckHandler
            let mut ack_resolver = ack_resolver.lock().unwrap();

            // Check if there are any addrs that have timed out. If so
            // remove them from the resolver, and push the message up
            let now = Instant::now();
            let mut client_addrs = Vec::new();
            for (k, v) in &client_ping_times {
                if (now - *v) > DEFAULT_DROP_TIME {
                    client_addrs.push(k.clone());
                }
            }

            for addr in client_addrs {
                ack_resolver.remove_client(addr);
                s.send(DroppedClient(addr)).unwrap();
                client_ping_times.remove(&addr);
                dropped_clients.insert(addr);
            }

            // Check if there are any ack resolvers which have timed out
            // if so, send them
            for res in ack_resolver.retrieve_timeouts().iter() {
                socket
                    .send_to(
                        &Type::Rel(res.index, res.msg.to_string()).serialize(),
                        res.addr,
                    )
                    .unwrap();
            }

            // If a datagram has been received be socket
            if let Ok((amt, addr)) = socket.recv_from(&mut buf) {

                if dropped_clients.contains(&addr) {
                    socket.send_to(&Type::Drop.serialize(), addr).unwrap();
                    continue;
                }

                // Convert the buffer into a string, and parse the
                // string as a DatagramType
                let buf = &buf[..amt];
                let msg = String::from_utf8(buf.to_vec()).unwrap();
                let datagram = Type::deserialize(&String::from(msg));
                println!("{:?}", datagram);

                client_ping_times.insert(addr, Instant::now());

                match datagram {
                    // Have the Transmitter send the relevant data
                    // to the Receiver
                    // Unreliable messages are simply forwarded
                    Type::Unrel(data) => s.send(ClientMessage(addr, data)).unwrap(),
                    // Reliable messages are compared with the AckResolver cache
                    // to determine if they are in order. If not, request a resend.
                    Type::Rel(ack_index, data) => {
                        let rel_result = ack_resolver.check_rel(addr, ack_index);

                        if rel_result == RelResult::NewRel {
                            s.send(ClientMessage(addr, data)).unwrap()
                        }
                        socket
                            .send_to(
                                &match rel_result {
                                    RelResult::NeedsRes => Type::Res,
                                    RelResult::ClientDropped => Type::Drop,
                                    _ => Type::Ack(ack_index),
                                }
                                .serialize(),
                                addr,
                            )
                            .unwrap();
                    }
                    // Ack messages are forwarded to the AckResolver,
                    // which accepts the ack, removing a resolver for the cache
                    Type::Ack(ack_index) => {
                        ack_resolver.accept_ack(addr, ack_index);
                    }
                    Type::Res => {
                        let resolvers = ack_resolver.resend_to(addr);
                        for res in resolvers {
                            socket
                                .send_to(
                                    &Type::Rel(res.index, res.msg.to_string()).serialize(),
                                    res.addr,
                                )
                                .unwrap();
                        }
                    }
                    Type::Ping => {
                        if client_ping_times.contains_key(&addr) {
                            client_ping_times.insert(addr, Instant::now());
                        }
                    }
                    _ => {}
                }
            }
            // Yield the thread (so it won't immediately lock the socket again)
            thread::yield_now();
        });
        r
    }

    ///
    /// Begins the transmitting loop for a concurrent `socket`, returning
    /// the `Sender` which can be used to send data through the `socket`
    ///
    fn transmit_to_clients_loop(
        socket: Arc<Mutex<UdpSocket>>,
        ack_resolver: Arc<Mutex<AckHandler>>,
        r_handler_state: Receiver<HandlerState>,
    ) -> Sender<SendPacket> {
        // Create the Sender and Receiver
        let (s, r): (Sender<SendPacket>, _) = unbounded();
        let mut state = HandlerState::Listening;

        // Spawn a new thread, and move the Receiver.
        // The thread undergoes an infinite loop, awaiting
        // datagrams that the server wishes to send
        std::thread::spawn(move || loop {
            state = r_handler_state.try_recv().unwrap_or(state);

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {}
            }

            // If Ok, determine which clients the
            // server wishes to send the datagrams to
            if let Ok(data) = r.try_recv() {
                let mut clients = Vec::new();
                let SendPacket { addrs, is_rel, msg } = data;

                for addr in addrs {
                    clients.push(addr);
                }

                let socket = socket.lock().unwrap();
                if is_rel {
                    let mut ack_resolver = ack_resolver.lock().unwrap();
                    for client in clients {
                        socket
                            .send_to(
                                &Type::Rel(
                                    ack_resolver.create_rel_resolver(client, msg.clone()),
                                    msg.clone(),
                                )
                                .serialize(),
                                client,
                            )
                            .unwrap();
                    }
                } else {
                    for client in clients {
                        socket
                            .send_to(&Type::Unrel(msg.clone()).serialize(), client)
                            .unwrap();
                    }
                }
                // Yield the thread (so it won't immediately lock the socket again)
                thread::yield_now();
            }
        });

        s
    }
}

impl Drop for DatagramHandler {
    // Ensure the listening / receiving threads are dropped
    // when the DatagramHandler leaves scope
    fn drop(&mut self) {
        self.s_to_clients_state.send(HandlerState::Dropped).unwrap();
        self.s_from_clients_state
            .send(HandlerState::Dropped)
            .unwrap();
    }
}
