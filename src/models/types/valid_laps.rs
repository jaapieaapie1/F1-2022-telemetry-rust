use std::io::Read;
use byteorder::ReadBytesExt;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct ValidLaps {
    pub bit_flags: u8,
}

impl ValidLaps {
    pub fn new<R: Read>(reader: &mut R) -> Result<ValidLaps, std::io::Error> {
        let bit_flags = reader.read_u8()?;
        Ok(ValidLaps {
            bit_flags,
        })
    }

    pub fn is_valid_lap(&self) -> bool {
        self.bit_flags & 0x01 == 0x01
    }

    pub fn is_valid_sector_1(&self) -> bool {
        self.bit_flags & 0x02 == 0x02
    }

    pub fn is_valid_sector_2(&self) -> bool {
        self.bit_flags & 0x04 == 0x04
    }

    pub fn is_valid_sector_3(&self) -> bool {
        self.bit_flags & 0x08 == 0x08
    }
}