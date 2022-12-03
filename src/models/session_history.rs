use std::io::Read;
use crate::models::PacketHeader;
use byteorder::{LittleEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use crate::models::enums::{ActualTyreCompound, VisualTyreCompound};
use crate::models::traits::Packet;
use crate::models::types::ValidLaps;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct LapHistory {
    pub lap_time: u32,
    pub sector_1_time: u16,
    pub sector_2_time: u16,
    pub sector_3_time: u16,
    pub lap_valid_bit_flags: ValidLaps,
}

impl LapHistory {
    pub fn new<R: Read>(reader: &mut R) -> Result<LapHistory, std::io::Error> {
        Ok(LapHistory {
            lap_time: reader.read_u32::<LittleEndian>()?,
            sector_1_time: reader.read_u16::<LittleEndian>()?,
            sector_2_time: reader.read_u16::<LittleEndian>()?,
            sector_3_time: reader.read_u16::<LittleEndian>()?,
            lap_valid_bit_flags: ValidLaps::new(reader)?,
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct TyreStintHistory {
    pub end_lap: u8,
    pub tyre_actual_compound: ActualTyreCompound,
    pub tyre_visual_compound: VisualTyreCompound,
}

impl TyreStintHistory {
    pub fn new<R: Read>(reader: &mut R) -> Result<TyreStintHistory, std::io::Error> {
        Ok(TyreStintHistory {
            end_lap: reader.read_u8()?,
            tyre_actual_compound: ActualTyreCompound::from_u8(reader.read_u8()?).unwrap(),
            tyre_visual_compound: VisualTyreCompound::from_u8(reader.read_u8()?).unwrap(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketSessionHistory {
    pub header: PacketHeader,
    pub car_idx: u8,
    pub num_laps: u8,
    pub num_tyre_stints: u8,
    pub best_lap_time_lap_num: u8,
    pub best_sector1_time_lap_num: u8,
    pub best_sector2_time_lap_num: u8,
    pub best_sector3_time_lap_num: u8,
    pub lap_history_data: Vec<LapHistory>,
    pub tyre_stints_history: Vec<TyreStintHistory>,
}

event_system::signal!(SessionHistorySignal<SessionHistoryReceiver, PacketSessionHistory> = []);

impl Packet for PacketSessionHistory {
    const PACKET_ID: u8 = 11;
    const PACKET_SIZE: usize = 1155;

    fn new<R: Read>(reader: &mut R) -> Result<PacketSessionHistory, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let car_idx = reader.read_u8()?;
        let num_laps = reader.read_u8()?;
        let num_tyre_stints = reader.read_u8()?;
        let best_lap_time_lap_num = reader.read_u8()?;
        let best_sector1_time_lap_num = reader.read_u8()?;
        let best_sector2_time_lap_num = reader.read_u8()?;
        let best_sector3_time_lap_num = reader.read_u8()?;
        let mut lap_history_data = Vec::new();
        for _ in 0..100 {
            lap_history_data.push(LapHistory::new(reader)?);
        }
        let mut tyre_stints_history = Vec::new();
        for _ in 0..8 {
            tyre_stints_history.push(TyreStintHistory::new(reader)?);
        }
        Ok(PacketSessionHistory {
            header,
            car_idx,
            num_laps,
            num_tyre_stints,
            best_lap_time_lap_num,
            best_sector1_time_lap_num,
            best_sector2_time_lap_num,
            best_sector3_time_lap_num,
            lap_history_data,
            tyre_stints_history,
        })
    }

    event_system::signal_fns!();
}