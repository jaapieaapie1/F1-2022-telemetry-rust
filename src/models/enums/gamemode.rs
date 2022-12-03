use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum GameMode {
    EventMode = 0,
    GrandPrix = 3,
    TimeTrial = 5,
    SplitScreen = 6,
    OnlineCustom = 7,
    OnlineLeague = 8,
    CareerInvitational = 11,
    ChampionshipInvitational = 12,
    Championship = 13,
    OnlineChampionship = 14,
    OnlineWeeklyEvent = 15,
    Career22 = 19,
    Career22Online = 20,
    Benchmark = 127,
    Unknown,
}