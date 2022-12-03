use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum ResultStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
    Finished = 3,
    DidNotFinish = 4,
    Disqualified = 5,
    NotClassified = 6,
    Retired = 7,
}