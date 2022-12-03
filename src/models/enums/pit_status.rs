use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum PitStatus {
    None = 0,
    Pitting = 1,
    InPitArea = 2,
}