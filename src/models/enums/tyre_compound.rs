use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum ActualTyreCompound {
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
    Inter = 7,
    Wet = 8,
    Dry = 9,
    WetF1Classic = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
    Unknown = 0,
}

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum VisualTyreCompound {
    Soft = 16,
    Medium = 17,
    Hard = 18,
    Intermediate = 7,
    Wet = 8,
    DryF1Classic = 9,
    WetF1Classic = 10,
    F2Wet = 15,
    F2SuperSoft = 19,
    F2Soft = 20,
    F2Medium = 21,
    F2Hard = 22,
    Unknown = 0,
}