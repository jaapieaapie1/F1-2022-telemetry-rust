use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum SafetyCarStatus {
    NoSafetyCar = 0,
    FullSafetyCar = 1,
    VirtualSafetyCar = 2,
    FormationLap = 3,
}