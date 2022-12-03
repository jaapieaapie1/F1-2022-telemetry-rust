use std::io::{Error, Read};
use crate::models::{PacketHeader, WheelsVector};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct CarSetup {
    pub front_wing: u8,
    pub rear_wing: u8,
    pub on_throttle: u8,
    pub off_throttle: u8,
    pub front_camber: f32,
    pub rear_camber: f32,
    pub front_toe: f32,
    pub rear_toe: f32,
    pub front_suspension: u8,
    pub rear_suspension: u8,
    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,
    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,
    pub brake_pressure: u8,
    pub brake_bias: u8,
    pub tyre_pressure: WheelsVector<f32>,
    pub ballast: u8,
    pub fuel_load: f32,
}

impl CarSetup {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<CarSetup, Error> {
        Ok(CarSetup {
            front_wing: reader.read_u8()?,
            rear_wing: reader.read_u8()?,
            on_throttle: reader.read_u8()?,
            off_throttle: reader.read_u8()?,
            front_camber: reader.read_f32::<LittleEndian>()?,
            rear_camber: reader.read_f32::<LittleEndian>()?,
            front_toe: reader.read_f32::<LittleEndian>()?,
            rear_toe: reader.read_f32::<LittleEndian>()?,
            front_suspension: reader.read_u8()?,
            rear_suspension: reader.read_u8()?,
            front_anti_roll_bar: reader.read_u8()?,
            rear_anti_roll_bar: reader.read_u8()?,
            front_suspension_height: reader.read_u8()?,
            rear_suspension_height: reader.read_u8()?,
            brake_pressure: reader.read_u8()?,
            brake_bias: reader.read_u8()?,
            tyre_pressure: WheelsVector::<f32>::read_from(reader)?,
            ballast: reader.read_u8()?,
            fuel_load: reader.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CarSetupPacket {
    pub header: PacketHeader,
    pub car_setups: Vec<CarSetup>,
}

event_system::signal!(CarSetupSignal<CarSetupReceiver, CarSetupPacket> = []);

impl Packet for CarSetupPacket {
    const PACKET_ID: u8 = 5;
    const PACKET_SIZE: usize = 1102;

    fn new<R: Read>(reader: &mut R) -> Result<CarSetupPacket, Error> {
        let header = PacketHeader::new(reader)?;
        let mut car_setups = Vec::with_capacity(22);
        for _ in 0..22 {
            car_setups.push(CarSetup::read_from(reader)?);
        }
        Ok(CarSetupPacket {
            header,
            car_setups,
        })
    }

    event_system::signal_fns!();
}