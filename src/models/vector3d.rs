use std::io::{Error, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D {
            x,
            y,
            z,
        }
    }
}

impl Vector3D<f32> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Vector3D<f32>, Error> {
        let x = reader.read_f32::<LittleEndian>()?;
        let y = reader.read_f32::<LittleEndian>()?;
        let z = reader.read_f32::<LittleEndian>()?;

        Ok(Vector3D::new(x, y, z))
    }
}

impl Vector3D<i16> {
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Vector3D<i16>, Error> {
        let x = reader.read_i16::<LittleEndian>()?;
        let y = reader.read_i16::<LittleEndian>()?;
        let z = reader.read_i16::<LittleEndian>()?;

        Ok(Vector3D::new(x, y, z))
    }
}