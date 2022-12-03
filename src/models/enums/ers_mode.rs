use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum ErsMode {
    None = 0,
    Low = 1,
    Medium = 2,
    Overtake = 3,
}