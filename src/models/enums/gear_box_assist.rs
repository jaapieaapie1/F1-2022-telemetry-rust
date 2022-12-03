use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum GearboxAssist {
    NoAssist = 1,
    SuggestedShift = 2,
    AutoShift = 3,
}