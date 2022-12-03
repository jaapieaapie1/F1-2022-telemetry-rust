use std::io::{Error, Read};
use crate::models::{PacketHeader, Vector3D, WheelsVector};
use byteorder::{ReadBytesExt, LittleEndian};
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use crate::models::traits::Packet;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct MotionPacket {
    pub header: PacketHeader,
    pub car_motion_data: [CarMotionData; 22],

    pub suspension_position: WheelsVector<f32>,
    pub suspension_velocity: WheelsVector<f32>,
    pub suspension_acceleration: WheelsVector<f32>,
    pub wheel_speed: WheelsVector<f32>,
    pub wheel_slip: WheelsVector<f32>,
    pub local_velocity: Vector3D<f32>,
    pub angular_velocity: Vector3D<f32>,
    pub angular_acceleration: Vector3D<f32>,
    pub front_wheels_angle: f32,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct CarMotionData {
    pub world_position: Vector3D<f32>,
    pub world_velocity: Vector3D<f32>,
    pub world_forward_dir: Vector3D<i16>,
    pub world_right_dir: Vector3D<i16>,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl CarMotionData {
    pub fn new<R: Read>(reader: &mut R) -> Result<CarMotionData, Error> {
        Ok(CarMotionData {
            world_position: Vector3D::<f32>::read_from(reader)?,
            world_velocity: Vector3D::<f32>::read_from(reader)?,
            world_forward_dir: Vector3D::<i16>::read_from(reader)?,
            world_right_dir: Vector3D::<i16>::read_from(reader)?,
            g_force_lateral: reader.read_f32::<LittleEndian>()?,
            g_force_longitudinal: reader.read_f32::<LittleEndian>()?,
            g_force_vertical: reader.read_f32::<LittleEndian>()?,
            yaw: reader.read_f32::<LittleEndian>()?,
            pitch: reader.read_f32::<LittleEndian>()?,
            roll: reader.read_f32::<LittleEndian>()?,
        })
    }
}

event_system::signal!(MotionSignal<MotionReceiver, MotionPacket> = []);

impl Packet for MotionPacket {

    const PACKET_ID: u8 = 0;
    const PACKET_SIZE: usize = 1464;


    fn new<R: Read>(mut reader: &mut R) -> Result<MotionPacket, Error> {
        let header = PacketHeader::new(reader)?;
        let mut car_motion_data: [CarMotionData; 22] = [CarMotionData {
            world_position: Vector3D {x: 0.0, y: 0.0, z: 0.0},
            world_velocity: Vector3D {x: 0.0, y: 0.0, z: 0.0},
            world_forward_dir: Vector3D {x: 0, y: 0, z: 0},
            world_right_dir: Vector3D {x: 0, y: 0, z: 0},
            g_force_lateral: 0.0,
            g_force_longitudinal: 0.0,
            g_force_vertical: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0
        }; 22];

        for motion_data in &mut car_motion_data {
            *motion_data = CarMotionData::new(reader)?;
        }

        let suspension_position = WheelsVector::<f32>::read_from(&mut reader)?;
        let suspension_velocity = WheelsVector::<f32>::read_from(&mut reader)?;
        let suspension_acceleration = WheelsVector::<f32>::read_from(&mut reader)?;
        let wheel_speed = WheelsVector::<f32>::read_from(&mut reader)?;
        let wheel_slip = WheelsVector::<f32>::read_from(&mut reader)?;
        let local_velocity = Vector3D::<f32>::read_from(&mut reader)?;
        let angular_velocity = Vector3D::<f32>::read_from(&mut reader)?;
        let angular_acceleration = Vector3D::<f32>::read_from(&mut reader)?;
        let front_wheels_angle = reader.read_f32::<LittleEndian>()?;

        Ok(MotionPacket {
            header,
            car_motion_data,
            suspension_position,
            suspension_velocity,
            suspension_acceleration,
            wheel_speed,
            wheel_slip,
            local_velocity,
            angular_velocity,
            angular_acceleration,
            front_wheels_angle,
        })
    }

    event_system::signal_fns!();
}