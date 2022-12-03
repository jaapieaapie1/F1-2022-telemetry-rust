use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}