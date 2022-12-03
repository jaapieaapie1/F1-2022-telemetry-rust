use std::fmt::Error;
use std::io::Read;
use crate::models::PacketHeader;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::models::enums::{InfringementType, PenaltyType};
use num_traits::FromPrimitive;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct FastestLap {
    pub vehicle_index: u8,
    pub lap_time: f32,
}

impl FastestLap {
    pub fn new<R: Read>(reader: &mut R) -> Result<FastestLap, std::io::Error> {
        Ok(FastestLap {
            vehicle_index: reader.read_u8()?,
            lap_time: reader.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct OnlyIndex {
    pub vehicle_index: u8,
}

impl OnlyIndex {
    pub fn new<R: Read>(reader: &mut R) -> Result<OnlyIndex, std::io::Error> {
        Ok(OnlyIndex {
            vehicle_index: reader.read_u8()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub vehicle_index: u8,
    pub other_vehicle_index: u8,
    pub time: u8,
    pub lap_num: u8,
    pub places_gained: u8,
}

impl Penalty {
    pub fn new<R: Read>(reader: &mut R) -> Result<Penalty, std::io::Error> {
        Ok(Penalty {
            penalty_type: PenaltyType::from_u8(reader.read_u8()?).unwrap(),
            infringement_type: InfringementType::from_u8(reader.read_u8()?).unwrap(),
            vehicle_index: reader.read_u8()?,
            other_vehicle_index: reader.read_u8()?,
            time: reader.read_u8()?,
            lap_num: reader.read_u8()?,
            places_gained: reader.read_u8()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct SpeedTrap {
    pub vehicle_index: u8,
    pub speed: f32,
    pub is_overall_fastest_in_session: bool,
    pub is_driver_fastest_in_session: bool,
    pub fastest_vehicle_idx_in_session: u8,
    pub fastest_speed_in_session: f32,
}

impl SpeedTrap {
    pub fn new<R: Read>(reader: &mut R) -> Result<SpeedTrap, std::io::Error> {
        Ok(SpeedTrap {
            vehicle_index: reader.read_u8()?,
            speed: reader.read_f32::<LittleEndian>()?,
            is_overall_fastest_in_session: reader.read_u8()? != 0,
            is_driver_fastest_in_session: reader.read_u8()? != 0,
            fastest_vehicle_idx_in_session: reader.read_u8()?,
            fastest_speed_in_session: reader.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct StartLights {
    pub num_lights: u8,
}

impl StartLights {
    pub fn new<R: Read>(reader: &mut R) -> Result<StartLights, std::io::Error> {
        Ok(StartLights {
            num_lights: reader.read_u8()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Flashback {
    pub frame_identifier: u32,
    pub session_time: f32,
}

impl Flashback {
    pub fn new<R: Read>(reader: &mut R) -> Result<Flashback, std::io::Error> {
        Ok(Flashback {
            frame_identifier: reader.read_u32::<LittleEndian>()?,
            session_time: reader.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Buttons {
    pub button_status: u32,
}

impl Buttons {
    pub fn new<R: Read>(reader: &mut R) -> Result<Buttons, std::io::Error> {
        Ok(Buttons {
            button_status: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum EventDetails {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(OnlyIndex),
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits(OnlyIndex),
    ChequeredFlag,
    RaceWinner(OnlyIndex),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    LightsOut,
    DriveThroughPenaltyServed(OnlyIndex),
    StopGoPenaltyServed(OnlyIndex),
    Flashback(Flashback),
    ButtonStatus(Buttons),
}

impl EventDetails {
    pub fn from_str<R: Read>(value: &str, reader: &mut R) -> Result<Self, Error> {
        Ok(match value {
            "SSTA" => EventDetails::SessionStarted,
            "SEND" => EventDetails::SessionEnded,
            "FTLP" => EventDetails::FastestLap(FastestLap::new(reader).unwrap()),
            "RTMT" => EventDetails::Retirement(OnlyIndex::new(reader).unwrap()),
            "DRSE" => EventDetails::DRSEnabled,
            "DRSD" => EventDetails::DRSDisabled,
            "TMPT" => EventDetails::TeamMateInPits(OnlyIndex::new(reader).unwrap()),
            "CHQF" => EventDetails::ChequeredFlag,
            "RCWN" => EventDetails::RaceWinner(OnlyIndex::new(reader).unwrap()),
            "PENA" => EventDetails::Penalty(Penalty::new(reader).unwrap()),
            "SPTP" => EventDetails::SpeedTrap(SpeedTrap::new(reader).unwrap()),
            "STLG" => EventDetails::StartLights(StartLights::new(reader).unwrap()),
            "LGOT" => EventDetails::LightsOut,
            "DTSV" => EventDetails::DriveThroughPenaltyServed(OnlyIndex::new(reader).unwrap()),
            "SGSV" => EventDetails::StopGoPenaltyServed(OnlyIndex::new(reader).unwrap()),
            "FLBK" => EventDetails::Flashback(Flashback::new(reader).unwrap()),
            "BUTN" => EventDetails::ButtonStatus(Buttons::new(reader).unwrap()),
            _ => return Err(Error::default()),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketEventData {
    pub header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub event_details: Option<EventDetails>,
    pub test: Vec<u8>,
}

event_system::signal!(EventSignal<EventReceiver, PacketEventData> = []);

impl Packet for PacketEventData {
    const PACKET_ID: u8 = 3;
    const PACKET_SIZE: usize = 40;

    fn new<R: Read>(reader: &mut R) -> Result<PacketEventData, std::io::Error> {
        let mut event_string_code = [0; 4];

        reader.read_exact(&mut event_string_code)?;
        let event_details = EventDetails::from_str(
            &String::from_utf8_lossy(&event_string_code),
            reader,
        ).ok();

        Ok(PacketEventData {
            header: PacketHeader::new(reader)?,
            event_string_code,
            event_details,
            test: vec![],
        })
    }

    event_system::signal_fns!();
}