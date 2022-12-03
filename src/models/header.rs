use std::io::Error;
use std::io::{
    Read,
};
use byteorder::{ReadBytesExt, LittleEndian};
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

impl PacketHeader {
    pub const PACKET_SIZE: usize = 24;

    pub fn new<R: Read>(reader: &mut R) -> Result<PacketHeader, Error> {
        Ok(PacketHeader {
            packet_format: reader.read_u16::<LittleEndian>()?,
            game_major_version: reader.read_u8()?,
            game_minor_version: reader.read_u8()?,
            packet_version: reader.read_u8()?,
            packet_id: reader.read_u8()?,
            session_uid: reader.read_u64::<LittleEndian>()?,
            session_time: reader.read_f32::<LittleEndian>()?,
            frame_identifier: reader.read_u32::<LittleEndian>()?,
            player_car_index: reader.read_u8()?,
            secondary_player_car_index: reader.read_u8()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;
    use byteorder::{LittleEndian, WriteBytesExt};

    #[test]
    fn it_works() {
        let mut buf = Buff::new();

        buf.write_u16::<LittleEndian>(2022).expect("Failed to write packet format");
        buf.write_u8(1).expect("Failed to write game major version");
        buf.write_u8(2).expect("Failed to write game minor version");
        buf.write_u8(3).expect("Failed to write packet version");
        buf.write_u8(4).expect("Failed to write packet id");
        buf.write_u64::<LittleEndian>(5).expect("Failed to write session uid");
        buf.write_f32::<LittleEndian>(6.0).expect("Failed to write session time");
        buf.write_u32::<LittleEndian>(7).expect("Failed to write frame identifier");
        buf.write_u8(8).expect("Failed to write player car index");
        buf.write_u8(9).expect("Failed to write secondary player car index");

        let header = PacketHeader::new(&mut buf).unwrap();
        assert_eq!(header.packet_format, 2022);
        assert_eq!(header.game_major_version, 1);
        assert_eq!(header.game_minor_version, 2);
        assert_eq!(header.packet_version, 3);
        assert_eq!(header.packet_id, 4);
        assert_eq!(header.session_uid, 5);
        assert_eq!(header.session_time, 6.0);
        assert_eq!(header.frame_identifier, 7);
        assert_eq!(header.player_car_index, 8);
        assert_eq!(header.secondary_player_car_index, 9);
    }

    struct Buff {
        pub buf: Vec<u8>,
        r_pos: u64,
        w_pos: u64
    }

    impl Buff {
        fn new() -> Buff {
            Buff {
                buf: vec![],
                r_pos: 0,
                w_pos: 0
            }
        }

        fn add_rpos(&mut self, val: u64) {
            self.r_pos += val;
        }

        fn add_wpos(&mut self, val: u64) {
            self.w_pos += val;
        }
    }

    impl Read for Buff {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut i = 0;
            while i < buf.len() {
                buf[i] = self.buf[self.r_pos as usize + i];
                i += 1;
            }

            self.add_rpos(i as u64);

            Ok(i)
        }
    }

    impl Write for Buff {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut i = 0;
            while i < buf.len() {
                self.buf.push(buf[i]);
                i += 1;
            }

            self.add_wpos(i as u64);

            Ok(i)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}