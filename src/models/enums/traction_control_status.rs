use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum TractionControlStatus {
    Off = 0,
    Medium = 1,
    Full = 2,
}