//! RPG Server - Datagram Handler
//!
//! Christian Schmid - April 2021
//! CS510 - Rust Programming

use super::{
    enums::{HandlerState, RelResult, SendTo},
    resolver::AckHandler,
    types::Type,
};

use crossbeam::channel::{unbounded, Receiver, Sender};

use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

///
/// A udp datagram handler, which recieves
/// the lowest-level byte data from incoming
/// clients.
///
/// The datagrams sent by clients are
/// parsed into their appropriate types, and handled
/// based on type (see datagram/type.rs)
///
pub struct DatagramHandler {
    // A crossbeam Sender, which server threads can
    // use to send clients datagrams
    s_to_clients: Sender<(SendTo, bool, String)>,
    // A crossbeam Receiver, which server threads can
    // use to listen for client datagrams
    r_from_clients: Receiver<(SocketAddr, String)>,

    // A crossbeam Sender, which allows the DatagramHandler
    // thread to communicate to its constituent threads
    // the state of the handler (ie. listening, or not listening,
    // aborting)
    s_handler_state: Sender<HandlerState>,
}

impl DatagramHandler {
    ///
    /// Creates a new udp socket reciever / listener, on specified `port`.
    /// Begins listening for datagrams from clients.
    ///
    pub fn new(port: u32) -> std::io::Result<Self> {
        // Attempt to create the UdpSocket.
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;

        // Set the UdpSocket's read timeout to 5 milliseconds,
        // to avoid blocking data being sent by the socket
        socket.set_read_timeout(Some(Duration::from_millis(5)))?;

        // Convert the socket to a Arc Mutex, for concurrency
        let socket = Arc::new(Mutex::new(socket));

        let ack_resolver = Arc::new(Mutex::new(AckHandler::new()));

        // Create the channels which will handle synchronizing
        // state between handler threads
        let (s_handler_state, r_handler_state) = unbounded();

        // Begin the thread where the socket recieves datagrams from
        // clients, parses them, and passes relevant information,
        // to other threads in the server
        let r_from_clients = Self::receive_clients_loop(
            socket.clone(),
            ack_resolver.clone(),
            r_handler_state.clone(),
        );

        // Begin the thread where the socket awaits other threads
        // in the server to send information to clients its
        // connected with
        let s_to_clients =
            Self::transmit_to_clients_loop(socket.clone(), ack_resolver.clone(), r_handler_state);

        Ok(Self {
            s_to_clients: s_to_clients.clone(),
            r_from_clients: r_from_clients.clone(),
            s_handler_state,
        })
    }

    ///
    /// Clones a `Sender` and `Receiver`, to be used in other systems
    ///
    pub fn get_sender_receiver(
        &self,
    ) -> (
        Sender<(SendTo, bool, String)>,
        Receiver<(SocketAddr, String)>,
    ) {
        (self.s_to_clients.clone(), self.r_from_clients.clone())
    }

    ///
    /// Starts or stops the `DatagramHandler`'s sending
    /// and receiving of data, determined by `is_listening`.
    ///
    pub fn set_listening(&mut self, is_listening: bool) {
        self.s_handler_state
            .send(if is_listening {
                HandlerState::Listening
            } else {
                HandlerState::Stopped
            })
            .unwrap();
    }

    ///
    /// Begins the receive loop for a concurrent `socket`, returning
    /// the `Reciever` which awaits messages from clients
    ///
    fn receive_clients_loop(
        socket: Arc<Mutex<UdpSocket>>,
        ack_resolver: Arc<Mutex<AckHandler>>,
        r_handler_state: Receiver<HandlerState>,
    ) -> Receiver<(SocketAddr, String)> {
        // Create the Sender and Receiver
        let (s, r): (_, Receiver<(SocketAddr, String)>) = unbounded();
        let mut state = HandlerState::Listening;

        // Spawn a new thread, and move the Sender.
        // The thread undergoes an infinite loop, awaiting
        // datagrams received by the socket
        std::thread::spawn(move || loop {
            if let Ok(new_state) = r_handler_state.recv_timeout(Duration::from_millis(5)) {
                state = new_state;
            }

            let mut buf = [0; 100];

            // Check if there are any ack resolvers which have timed out
            // if so, send them
            for resolver in ack_resolver.lock().unwrap().retrieve_timeouts() {
                socket
                    .lock()
                    .unwrap()
                    .send_to(
                        &Type::to_buffer(Type::Rel(resolver.index, resolver.msg.to_string())),
                        resolver.addr,
                    )
                    .unwrap();
            }

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {
                    // Retrieve a lock on the sockets, to be used
                    // throughout the whole block
                    if let Ok(sock) = socket.lock() {
                        // If a datagram has been received be socket
                        if let Ok((amt, src)) = sock.recv_from(&mut buf) {
                            // Convert the buffer into a string, and parse the
                            // string as a DatagramType
                            let buf = &buf[..amt];
                            let datagram =
                                Type::from_str(&String::from_utf8(buf.to_vec()).unwrap()).unwrap();

                            match datagram {
                                // Have the Transmitter send the relevant data
                                // to the Receiver
                                // Unreliable messages are simply forwarded
                                Type::Unrel(data) => s.send((src, data)).unwrap(),
                                // Reliable messages are compared with the AckResolver cache
                                // to determine if they are in order. If not, request a resend.
                                Type::Rel(ack_index, data) => {
                                    let rel_result =
                                        ack_resolver.lock().unwrap().check_rel(src, ack_index);
                                    if rel_result == RelResult::NewRel {
                                        s.send((src, data)).unwrap()
                                    }
                                    sock.send_to(
                                        &Type::to_buffer(if rel_result == RelResult::NeedsRes {
                                            Type::Res
                                        } else {
                                            Type::Ack(ack_index)
                                        }),
                                        src,
                                    )
                                    .unwrap();
                                }
                                // Ack messages are forwarded to the AckResolver,
                                // which accepts the ack, removing a resolver for the cache
                                Type::Ack(ack_index) => ack_resolver
                                    .lock()
                                    .unwrap()
                                    .accept_ack(src, ack_index)
                                    .unwrap(),
                                Type::Res => {}
                            }
                        }
                    }
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
    ) -> Sender<(SendTo, bool, String)> {
        // Create the Sender and Receiver
        let (s, r): (_, Receiver<(SendTo, bool, String)>) = unbounded();
        let mut state = HandlerState::Listening;

        // Spawn a new thread, and move the Receiver.
        // The thread undergoes an infinite loop, awaiting
        // datagrams that the server wishes to send
        std::thread::spawn(move || loop {
            if let Ok(new_state) = r_handler_state.recv_timeout(Duration::from_millis(5)) {
                state = new_state;
            }

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {
                    // If Ok, determine which clients the
                    // server wishes to send the datagrams to
                    if let Ok(data) = r.recv_timeout(Duration::from_millis(5)) {
                        match data.0 {
                            SendTo::One(addr) => {
                                let data_buffer = match data.1 {
                                    true => Type::to_buffer(Type::Rel(
                                        ack_resolver
                                            .lock()
                                            .unwrap()
                                            .create_rel_resolver(addr, data.2.clone()),
                                        data.2,
                                    )),
                                    false => Type::to_buffer(Type::Unrel(data.2)),
                                };
                                socket
                                    .lock()
                                    .unwrap()
                                    .send_to(&data_buffer, addr)
                                    .expect("Failed to send data through the server socket");
                            }
                            SendTo::AllBut(_) => {}
                            SendTo::All => {}
                        }
                    }
                }
            }
        });

        s
    }
}

impl Drop for DatagramHandler {
    // Ensure the listening / receiving threads are dropped
    // when the DatagramHandler leaves scope
    fn drop(&mut self) {
        self.s_handler_state.send(HandlerState::Dropped).unwrap();
    }
}
