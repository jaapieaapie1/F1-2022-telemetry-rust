use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum ReadyStatus {
    NotReady = 0,
    Ready = 1,
    Spectating = 2,
}