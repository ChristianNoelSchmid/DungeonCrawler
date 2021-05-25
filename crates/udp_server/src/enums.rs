//! General Enums for UdpHandler
//! 
//! Christian Schmid 2021
//!

///
/// Used to synchronize the UdpHandler with its
/// constituent threads. The send/receive threads
/// respond accordingly to an update in this state.
///
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HandlerState {
    Listening,
    Stopped,
    Dropped,
}

///
/// A result from a RelHandler
#[derive(PartialEq, Eq, Debug)]
pub enum RelResult {
    NewRel,
    RepeatedRel,
    NeedsRes,
    ClientDropped,
}
