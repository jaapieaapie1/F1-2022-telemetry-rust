use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum Sector {
    Sector1 = 0,
    Sector2 = 1,
    Sector3 = 2,
    Unknown,
}