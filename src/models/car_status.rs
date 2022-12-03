use std::io::Read;
use crate::models::enums::{ActualTyreCompound, ErsMode, FuelMix, TractionControlStatus, VehicleFiaFlags, VisualTyreCompound};
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::PacketHeader;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct CarStatusData {
    pub traction_control: TractionControlStatus,
    pub anti_lock_brakes: bool,
    pub fuel_mix: FuelMix,
    pub front_brake_bias: u8,
    pub pit_limiter_status: bool,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_remaining_laps: f32,
    pub max_rpm: u16,
    pub idle_rpm: u16,
    pub max_gears: u8,
    pub drs_allowed: bool,
    pub drs_activation_distance: u16,
    pub actual_tyre_compound: ActualTyreCompound,
    pub visual_tyre_compound: VisualTyreCompound,
    pub tyres_age_laps: u8,
    pub vehicle_fia_flags: VehicleFiaFlags,
    pub ers_store_energy: f32,
    pub ers_deploy_mode: ErsMode,
    pub ers_harvested_this_lap_mguk: f32,
    pub ers_harvested_this_lap_mguh: f32,
    pub ers_deployed_this_lap: f32,
    pub network_paused: bool,
}

impl CarStatusData {
    pub fn new<R: Read>(reader: &mut R) -> Result<CarStatusData, std::io::Error> {
        Ok(CarStatusData {
            traction_control: TractionControlStatus::from_u8(reader.read_u8()?).unwrap(),
            anti_lock_brakes: reader.read_u8()? != 0,
            fuel_mix: FuelMix::from_u8(reader.read_u8()?).unwrap(),
            front_brake_bias: reader.read_u8()?,
            pit_limiter_status: reader.read_u8()? != 0,
            fuel_in_tank: reader.read_f32::<LittleEndian>()?,
            fuel_capacity: reader.read_f32::<LittleEndian>()?,
            fuel_remaining_laps: reader.read_f32::<LittleEndian>()?,
            max_rpm: reader.read_u16::<LittleEndian>()?,
            idle_rpm: reader.read_u16::<LittleEndian>()?,
            max_gears: reader.read_u8()?,
            drs_allowed: reader.read_u8()? != 0,
            drs_activation_distance: reader.read_u16::<LittleEndian>()?,
            actual_tyre_compound: ActualTyreCompound::from_u8(reader.read_u8()?).unwrap(),
            visual_tyre_compound: VisualTyreCompound::from_u8(reader.read_u8()?).unwrap(),
            tyres_age_laps: reader.read_u8()?,
            vehicle_fia_flags: VehicleFiaFlags::from_u8(reader.read_u8()?).unwrap(),
            ers_store_energy: reader.read_f32::<LittleEndian>()?,
            ers_deploy_mode: ErsMode::from_u8(reader.read_u8()?).unwrap(),
            ers_harvested_this_lap_mguk: reader.read_f32::<LittleEndian>()?,
            ers_harvested_this_lap_mguh: reader.read_f32::<LittleEndian>()?,
            ers_deployed_this_lap: reader.read_f32::<LittleEndian>()?,
            network_paused: reader.read_u8()? != 0,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketCarStatus {
    pub header: PacketHeader,
    pub car_status_data: Vec<CarStatusData>,
}

event_system::signal!(CarStatusSignal<CarStatusReceiver, PacketCarStatus> = []);

impl Packet for PacketCarStatus {
    const PACKET_ID: u8 = 7;
    const PACKET_SIZE: usize = 1058;

    fn new<R: Read>(reader: &mut R) -> Result<PacketCarStatus, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let mut car_status_data = Vec::new();
        for _ in 0..22 {
            car_status_data.push(CarStatusData::new(reader)?);
        }
        Ok(PacketCarStatus {
            header,
            car_status_data,
        })
    }

    event_system::signal_fns!();
}