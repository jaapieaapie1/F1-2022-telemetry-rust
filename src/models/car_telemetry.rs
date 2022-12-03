use std::io::Read;
use crate::models::enums::{MdfPanel, SurfaceType};
use crate::models::{PacketHeader, WheelsVector};
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub rev_lights_bitfield: u16,
    pub brakes_temperature: WheelsVector<u16>,
    pub tyres_surface_temperature: WheelsVector<u8>,
    pub tyres_inner_temperature: WheelsVector<u8>,
    pub engine_temperature: u16,
    pub tyres_pressure: WheelsVector<f32>,
    pub surface_type: WheelsVector<SurfaceType>,
}

impl CarTelemetryData {
    pub fn new<R: Read>(reader: &mut R) -> Result<CarTelemetryData, std::io::Error> {
        let speed = reader.read_u16::<LittleEndian>()?;
        let throttle = reader.read_f32::<LittleEndian>()?;
        let steer = reader.read_f32::<LittleEndian>()?;
        let brake = reader.read_f32::<LittleEndian>()?;
        let clutch = reader.read_u8()?;
        let gear = reader.read_i8()?;
        let engine_rpm = reader.read_u16::<LittleEndian>()?;
        let drs = reader.read_u8()? != 0;
        let rev_lights_percent = reader.read_u8()?;
        let rev_lights_bitfield = reader.read_u16::<LittleEndian>()?;
        let brakes_temperature = WheelsVector::<u16>::read_from(reader)?;
        let tyres_surface_temperature = WheelsVector::<u8>::read_from(reader)?;
        let tyres_inner_temperature = WheelsVector::<u8>::read_from(reader)?;
        let engine_temperature = reader.read_u16::<LittleEndian>()?;
        let tyres_pressure = WheelsVector::<f32>::read_from(reader)?;
        let surface_type = WheelsVector::<SurfaceType>::read_from(reader)?;

        Ok(CarTelemetryData {
            speed,
            throttle,
            steer,
            brake,
            clutch,
            gear,
            engine_rpm,
            drs,
            rev_lights_percent,
            rev_lights_bitfield,
            brakes_temperature,
            tyres_surface_temperature,
            tyres_inner_temperature,
            engine_temperature,
            tyres_pressure,
            surface_type,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CarTelemetryPacket {
    pub header: PacketHeader,
    pub car_telemetry_data: Vec<CarTelemetryData>,
    pub mdf_panel_index: MdfPanel,
    pub mdf_panel_index_secondary_player: MdfPanel,
    pub suggested_gear: i8,
}

event_system::signal!(CarTelemetrySignal<CarTelemetryReceiver, CarTelemetryPacket> = []);

impl Packet for CarTelemetryPacket {
    const PACKET_ID: u8 = 6;
    const PACKET_SIZE: usize = 1347;

    fn new<R: Read>(reader: &mut R) -> Result<CarTelemetryPacket, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let mut car_telemetry_data = Vec::new();
        for _ in 0..22 {
            car_telemetry_data.push(CarTelemetryData::new(reader)?);
        }
        let mdf_panel_index = MdfPanel::from_u8(reader.read_u8()?).unwrap();

        let mdf_panel_index_secondary_player = MdfPanel::from_u8(reader.read_u8()?).unwrap();
        let suggested_gear = reader.read_i8()?;
        Ok(CarTelemetryPacket {
            header,
            car_telemetry_data,
            mdf_panel_index,
            mdf_panel_index_secondary_player,
            suggested_gear,
        })
    }

    event_system::signal_fns!();
}