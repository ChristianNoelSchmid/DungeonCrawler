#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HandlerState {
    Listening,
    Stopped,
    Dropped,
}

#[derive(PartialEq, Eq, Debug)]
pub enum RelResult {
    NewRel,
    RepeatedRel,
    NeedsRes,
    ClientDropped,
}
