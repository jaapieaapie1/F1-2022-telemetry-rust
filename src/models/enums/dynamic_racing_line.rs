use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum DynamicRacingLine {
    None = 0,
    Corners = 1,
    All = 2,
}

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum DynamicRacingLineType {
    L2D = 0,
    L3D = 1,
}