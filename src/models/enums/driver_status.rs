use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum DriverStatus {
    InGarage = 0,
    FlyingLap = 1,
    InLap = 2,
    OutLap = 3,
    OnTrack = 4,
}