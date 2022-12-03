use std::io::Read;
use crate::models::{PacketHeader, WheelsVector};
use byteorder::ReadBytesExt;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct CarDamage {
    pub tyre_wear: WheelsVector<f32>,
    pub tyre_damage: WheelsVector<u8>,
    pub brakes_damage: WheelsVector<u8>,
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub floor_damage: u8,
    pub diffuser_damage: u8,
    pub sidepod_damage: u8,
    pub drs_fault: bool,
    pub ers_fault: bool,
    pub gear_box_damage: u8,
    pub engine_damage: u8,
    pub engine_mguh_wear: u8,
    pub engine_es_wear: u8,
    pub engine_ce_wear: u8,
    pub engine_ice_wear: u8,
    pub engine_mguk_wear: u8,
    pub engine_tc_wear: u8,
    pub engine_blown: bool,
    pub engine_seized: bool,
}

impl CarDamage {
    pub fn new<R: Read>(reader: &mut R) -> Result<CarDamage, std::io::Error> {
        Ok(CarDamage {
            tyre_wear: WheelsVector::<f32>::read_from(reader)?,
            tyre_damage: WheelsVector::<u8>::read_from(reader)?,
            brakes_damage: WheelsVector::<u8>::read_from(reader)?,
            front_left_wing_damage: reader.read_u8()?,
            front_right_wing_damage: reader.read_u8()?,
            rear_wing_damage: reader.read_u8()?,
            floor_damage: reader.read_u8()?,
            diffuser_damage: reader.read_u8()?,
            sidepod_damage: reader.read_u8()?,
            drs_fault: reader.read_u8()? != 0,
            ers_fault: reader.read_u8()? != 0,
            gear_box_damage: reader.read_u8()?,
            engine_damage: reader.read_u8()?,
            engine_mguh_wear: reader.read_u8()?,
            engine_es_wear: reader.read_u8()?,
            engine_ce_wear: reader.read_u8()?,
            engine_ice_wear: reader.read_u8()?,
            engine_mguk_wear: reader.read_u8()?,
            engine_tc_wear: reader.read_u8()?,
            engine_blown: reader.read_u8()? != 0,
            engine_seized: reader.read_u8()? != 0,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketCarDamage {
    pub header: PacketHeader,
    pub car_damage_data: Vec<CarDamage>,
}

event_system::signal!(CarDamageSignal<CarDamageReceiver, PacketCarDamage> = []);

impl Packet for PacketCarDamage {
    const PACKET_ID: u8 = 10;
    const PACKET_SIZE: usize = 948;

    fn new<R: Read>(reader: &mut R) -> Result<PacketCarDamage, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let mut car_damage_data = Vec::new();
        for _ in 0..22 {
            car_damage_data.push(CarDamage::new(reader)?);
        }
        Ok(PacketCarDamage {
            header,
            car_damage_data,
        })
    }

    event_system::signal_fns!();
}