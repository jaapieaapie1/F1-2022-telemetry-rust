use std::io::Read;
use crate::models::enums::{DriverStatus, PitStatus, ResultStatus, Sector};
use crate::models::PacketHeader;
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct LapData {
    pub last_lap_time: u32,
    pub current_lap_time: u32,
    pub sector1_time: u16,
    pub sector2_time: u16,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub num_pit_stops: u8,
    pub sector: Sector,
    pub current_lap_invalid: bool,
    pub penalties: u8,
    pub warnings: u8,
    pub num_unserved_drive_through_penalties: u8,
    pub num_unserved_stop_go_penalties: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
    pub pit_lane_timer_active: bool,
    pub pit_lane_time_in_lane: u16,
    pub pit_stop_timer: u16,
    pub pit_stop_should_serve_penalty: bool,
}

impl LapData {
    pub fn new<R: Read>(reader: &mut R) -> Result<LapData, std::io::Error> {
        Ok(LapData {
            last_lap_time: reader.read_u32::<LittleEndian>()?,
            current_lap_time: reader.read_u32::<LittleEndian>()?,
            sector1_time: reader.read_u16::<LittleEndian>()?,
            sector2_time: reader.read_u16::<LittleEndian>()?,
            lap_distance: reader.read_f32::<LittleEndian>()?,
            total_distance: reader.read_f32::<LittleEndian>()?,
            safety_car_delta: reader.read_f32::<LittleEndian>()?,
            car_position: reader.read_u8()?,
            current_lap_num: reader.read_u8()?,
            pit_status: PitStatus::from_u8(reader.read_u8()?).unwrap(),
            num_pit_stops: reader.read_u8()?,
            sector: Sector::from_u8(reader.read_u8()?).unwrap(),
            current_lap_invalid: reader.read_u8()? != 0,
            penalties: reader.read_u8()?,
            warnings: reader.read_u8()?,
            num_unserved_drive_through_penalties: reader.read_u8()?,
            num_unserved_stop_go_penalties: reader.read_u8()?,
            grid_position: reader.read_u8()?,
            driver_status: DriverStatus::from_u8(reader.read_u8()?).unwrap(),
            result_status: ResultStatus::from_u8(reader.read_u8()?).unwrap(),
            pit_lane_timer_active: reader.read_u8()? != 0,
            pit_lane_time_in_lane: reader.read_u16::<LittleEndian>()?,
            pit_stop_timer: reader.read_u16::<LittleEndian>()?,
            pit_stop_should_serve_penalty: reader.read_u8()? != 0,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketLapData {
    pub header: PacketHeader,
    pub lap_data: Vec<LapData>,
    pub time_trial_pb_car_idx: u8,
    pub time_trial_rival_car_idx: u8,
}

event_system::signal!(LapDataSignal<LapDataReceiver, PacketLapData> = []);

impl Packet for PacketLapData {
    const PACKET_ID: u8 = 2;
    const PACKET_SIZE: usize = 972;

    fn new<R: Read>(reader: &mut R) -> Result<PacketLapData, std::io::Error> {
        let mut lap_data = vec![];
        for _ in 0..22 {
            lap_data.push(LapData::new(reader)?);
        }
        Ok(PacketLapData {
            header: PacketHeader::new(reader)?,
            lap_data,
            time_trial_pb_car_idx: reader.read_u8()?,
            time_trial_rival_car_idx: reader.read_u8()?,
        })
    }

    event_system::signal_fns!();
}