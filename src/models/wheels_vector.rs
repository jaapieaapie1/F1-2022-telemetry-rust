use std::io::Error;
use std::io::Read;
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::enums::SurfaceType;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct WheelsVector<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

impl WheelsVector<f32> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<WheelsVector<f32>, Error> {
        Ok(WheelsVector {
            rear_left: reader.read_f32::<LittleEndian>()?,
            rear_right: reader.read_f32::<LittleEndian>()?,
            front_left: reader.read_f32::<LittleEndian>()?,
            front_right: reader.read_f32::<LittleEndian>()?,
        })
    }
}

impl WheelsVector<u16> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<WheelsVector<u16>, Error> {
        Ok(WheelsVector {
            rear_left: reader.read_u16::<LittleEndian>()?,
            rear_right: reader.read_u16::<LittleEndian>()?,
            front_left: reader.read_u16::<LittleEndian>()?,
            front_right: reader.read_u16::<LittleEndian>()?,
        })
    }
}

impl WheelsVector<u8> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<WheelsVector<u8>, Error> {
        Ok(WheelsVector {
            rear_left: reader.read_u8()?,
            rear_right: reader.read_u8()?,
            front_left: reader.read_u8()?,
            front_right: reader.read_u8()?,
        })
    }
}

impl<T> WheelsVector<T> {
    pub fn new(rear_left: T, rear_right: T, front_left: T, front_right: T) -> WheelsVector<T> {
        WheelsVector {
            rear_left,
            rear_right,
            front_left,
            front_right,
        }
    }
}

impl WheelsVector<SurfaceType> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<WheelsVector<SurfaceType>, Error> {
        Ok(WheelsVector {
            rear_left: SurfaceType::from_u8(reader.read_u8()?).unwrap(),
            rear_right: SurfaceType::from_u8(reader.read_u8()?).unwrap(),
            front_left: SurfaceType::from_u8(reader.read_u8()?).unwrap(),
            front_right: SurfaceType::from_u8(reader.read_u8()?).unwrap(),
        })
    }
}