use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum MdfPanel {
    CarSetup = 0,
    Pits = 1,
    Damage = 2,
    Engine = 3,
    Temperatures = 4,
}