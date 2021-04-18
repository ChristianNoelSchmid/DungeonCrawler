use super::{
    resolver::AckResolverHandler, 
    enums::{SendTo, HandlerState}
};

use crossbeam::channel::{unbounded, Receiver, Sender};

use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};

///
/// A udp datagram handler, which recieves
/// the lowest-level byte data from incoming
/// clients.
///
/// The datagrams sent by clients are
/// parsed into their appropriate types, and handled
/// based on type (see datagram_type.rs)
///
pub struct DatagramHandler {
    s_to_clients: Sender<(SendTo, String)>,
    s_handler_state: Sender<HandlerState>,
}

impl DatagramHandler {
    ///
    /// Creates a new udp socket reciever / listener, on specified `port`.
    /// Returns a 3-ple: the new `DatagramHandler`, and its `Sender` and
    /// `Receiver` mpsc channels, used to communicate with other services.
    ///
    pub fn new(port: u32) -> std::io::Result<(Self, Receiver<(SocketAddr, String)>)> {
        // Attempt to create the UdpSocket.
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;

        // Set the UdpSocket's read timeout to 5 milliseconds,
        // to avoid blocking data being sent by the socket
        socket.set_read_timeout(Some(Duration::from_millis(5)))?;

        // Convert the socket to a Arc Mutex, for concurrency
        let socket = Arc::new(Mutex::new(socket));

        // Create the channels which will handle synchronizing
        // state between handler threads
        let (s_handler_state, r_handler_state) = unbounded();

        let rx_from_clients = Self::receive_clients_loop(socket.clone(), r_handler_state.clone());
        let tx_to_clients = Self::transmit_to_clients_loop(socket.clone(), r_handler_state);

        // Return the new handler, with its UdpSocket wrapped
        // in a reference counter and mutex, for concurrency
        Ok((
            Self {
                s_to_clients: tx_to_clients.clone(),
                s_handler_state,
            },
            rx_from_clients,
        ))
    }

    pub fn set_listening(&mut self, is_listening: bool) {
        if is_listening {
            self.s_handler_state.send(HandlerState::Listening).unwrap();
        } else {
            self.s_handler_state.send(HandlerState::Stopped).unwrap();
        };
    }

    pub fn get_sender(&self) -> Sender<(SendTo, String)> {
        self.s_to_clients.clone()
    }

    ///
    /// Begins the receive loop for a concurrent `socket`, returning
    /// the `Reciever` which awaits messages from clients
    ///
    fn receive_clients_loop(
        socket: Arc<Mutex<UdpSocket>>,
        rx_recieve_loop: Receiver<HandlerState>,
    ) -> Receiver<(SocketAddr, String)> {
        // Create the Sender and Receiver
        let (tx, rx): (_, Receiver<(SocketAddr, String)>) = unbounded();
        let mut state = HandlerState::Listening;

        // Spawn a new thread, and move the Sender.
        // The thread undergoes an infinite loop, awaiting
        // datagrams received by the socket
        std::thread::spawn(move || loop {
            if let Ok(val) = rx_recieve_loop.recv_timeout(Duration::from_millis(5)) {
                state = val;
            }

            let mut buf = [0; 100];

            match state {
                HandlerState::Dropped => break,
                HandlerState::Stopped => continue,
                HandlerState::Listening => {
                    // If a datagram has been received in the last 5 milliseconds
                    if let Ok((amt, src)) = socket.lock().unwrap().recv_from(&mut buf) {
                        let buf = &buf[..amt];

                        // Have the Transmitter send the relevant data
                        // to the Receiver
                        tx.send((src, String::from_utf8(buf.to_vec()).unwrap()))
                            .unwrap();
                    }
                }
            }
            // Yield the thread (so it won't immediately lock the socket again)
            thread::yield_now();
        });
        rx
    }

    ///
    /// Begins the transmitting loop for a concurrent `socket`, returning
    /// the `Sender` which can be used to send data through the `socket`
    ///
    fn transmit_to_clients_loop(
        socket: Arc<Mutex<UdpSocket>>,
        rx_send_loop: Receiver<HandlerState>,
    ) -> Sender<(SendTo, String)> {
        // Create the Sender and Receiver
        let (s, r): (_, Receiver<(SendTo, String)>) = unbounded();
        let mut state = HandlerState::Listening;

        // Spawn a new thread, and move the Receiver.
        // The thread undergoes an infinite loop, awaiting
        // datagrams that the server wishes to send
        std::thread::spawn(move || loop {
            if let Ok(val) = rx_send_loop.recv_timeout(Duration::from_millis(5)) {
                state = val;
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
                                socket
                                    .lock()
                                    .unwrap()
                                    .send_to(data.1.as_bytes(), addr)
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