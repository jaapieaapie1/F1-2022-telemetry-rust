use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum SessionType {
    Unknown = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    ShortP = 4,
    Q1 = 5,
    Q2 = 6,
    Q3 = 7,
    ShortQ = 8,
    OSQ = 9,
    R = 10,
    R2 = 11,
    TimeTrial = 12,
}