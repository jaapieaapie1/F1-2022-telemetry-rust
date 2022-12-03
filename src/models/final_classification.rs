use std::io::Read;
use crate::models::PacketHeader;
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::enums::{ActualTyreCompound, ResultStatus, VisualTyreCompound};
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ClassificationData {
    pub position: u8,
    pub num_laps: u8,
    pub grid_position: u8,
    pub points: u8,
    pub num_pit_stops: u8,
    pub result_status: ResultStatus,
    pub best_lap_time: u32,
    pub total_race_time: f64,
    pub penalties_time: u8,
    pub num_penalties: u8,
    pub num_tyre_stints: u8,
    pub tyre_stints_actual: Vec<ActualTyreCompound>,
    pub tyre_stints_visual: Vec<VisualTyreCompound>,
}

impl ClassificationData {
    pub fn new<R: Read>(reader: &mut R) -> Result<ClassificationData, std::io::Error> {
        Ok(ClassificationData {
            position: reader.read_u8()?,
            num_laps: reader.read_u8()?,
            grid_position: reader.read_u8()?,
            points: reader.read_u8()?,
            num_pit_stops: reader.read_u8()?,
            result_status: ResultStatus::from_u8(reader.read_u8()?).unwrap(),
            best_lap_time: reader.read_u32::<LittleEndian>()?,
            total_race_time: reader.read_f64::<LittleEndian>()?,
            penalties_time: reader.read_u8()?,
            num_penalties: reader.read_u8()?,
            num_tyre_stints: reader.read_u8()?,
            tyre_stints_actual: {
                let mut v = Vec::new();
                for _ in 0..8 {
                    v.push(ActualTyreCompound::from_u8(reader.read_u8()?).unwrap());
                }
                v
            },
            tyre_stints_visual: {
                let mut v = Vec::new();
                for _ in 0..8 {
                    v.push(VisualTyreCompound::from_u8(reader.read_u8()?).unwrap());
                }
                v
            },
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader,
    pub num_cars: u8,
    pub classification_data: Vec<ClassificationData>,
}

event_system::signal!(FinalClassificationSignal<FinalClassificationReceiver, PacketFinalClassificationData> = []);

impl Packet for PacketFinalClassificationData {
    const PACKET_ID: u8 = 8;
    const PACKET_SIZE: usize = 1015;

    fn new<R: Read>(reader: &mut R) -> Result<PacketFinalClassificationData, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let num_cars = reader.read_u8()?;
        let mut classification_data = Vec::new();
        for _ in 0..22 {
            classification_data.push(ClassificationData::new(reader)?);
        }
        Ok(PacketFinalClassificationData {
            header,
            num_cars,
            classification_data,
        })
    }

    event_system::signal_fns!();
}