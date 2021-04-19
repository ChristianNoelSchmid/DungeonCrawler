//! RPG Server - Datagram Handler
//! 
//! Christian Schmid - April 2021
//! CS510 - Rust Programming 

use super::{
    enums::{HandlerState, RelResult, SendTo},
    resolver::AckHandler,
    types::DatagramType,
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
    s_to_clients: Sender<(SendTo, bool, String)>,
    r_from_clients: Receiver<(SocketAddr, String)>,

    s_handler_state: Sender<HandlerState>,
}

impl DatagramHandler {
    ///
    /// Creates a new udp socket reciever / listener, on specified `port`.
    /// Returns a 3-ple: the new `DatagramHandler`, and its `Sender` and
    /// `Receiver` mpsc channels, used to communicate with other services.
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

        let r_from_clients = Self::receive_clients_loop(
            socket.clone(),
            ack_resolver.clone(),
            r_handler_state.clone(),
        );
        let s_to_clients =
            Self::transmit_to_clients_loop(socket.clone(), ack_resolver.clone(), r_handler_state);

        // Return the new handler, with its UdpSocket wrapped
        // in a reference counter and mutex, for concurrency
        Ok(Self {
            s_to_clients: s_to_clients.clone(),
            r_from_clients: r_from_clients.clone(),
            s_handler_state,
        })
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
                        &DatagramType::to_buffer(DatagramType::Rel(
                            resolver.index,
                            resolver.msg.to_string(),
                        )),
                        resolver.addr,
                    )
                    .unwrap();
            }

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {
                    // If a datagram has been received
                    if let Ok(sock) = socket.lock() {
                        if let Ok((amt, src)) = sock.recv_from(&mut buf) {
                            let buf = &buf[..amt];
                            let datagram =
                                DatagramType::from_str(&String::from_utf8(buf.to_vec()).unwrap())
                                    .unwrap();

                            match datagram {
                                // Have the Transmitter send the relevant data
                                // to the Receiver
                                DatagramType::Unrel(data) => s.send((src, data)).unwrap(),
                                DatagramType::Rel(ack_index, data) => {
                                    let rel_result =
                                        ack_resolver.lock().unwrap().check_rel(src, ack_index);
                                    if rel_result == RelResult::NewRel {
                                        s.send((src, data)).unwrap()
                                    }
                                    sock.send_to(
                                        &DatagramType::to_buffer(
                                            if rel_result == RelResult::NeedsRes {
                                                DatagramType::Res
                                            } else {
                                                DatagramType::Ack(ack_index)
                                            },
                                        ),
                                        src,
                                    )
                                    .unwrap();
                                }
                                DatagramType::Ack(ack_index) => ack_resolver
                                    .lock()
                                    .unwrap()
                                    .accept_ack(src, ack_index)
                                    .unwrap(),
                                DatagramType::Res => {}
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
                                    true => DatagramType::to_buffer(DatagramType::Rel(
                                        ack_resolver
                                            .lock()
                                            .unwrap()
                                            .create_rel_resolver(addr, data.2.clone()),
                                        data.2,
                                    )),
                                    false => DatagramType::to_buffer(DatagramType::Unrel(data.2)),
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
    fn drop(&mut self) {
        self.s_handler_state.send(HandlerState::Dropped).unwrap();
    }
}
