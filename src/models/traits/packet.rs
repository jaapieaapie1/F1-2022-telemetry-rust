use std::io::Read;

pub trait Packet {
    const PACKET_ID: u8;
    const PACKET_SIZE: usize;

    fn new<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> where Self: Sized;

    fn get_packet_size() -> usize {
        Self::PACKET_SIZE
    }

    fn emit(self);

    fn listen(callable: &'static dyn Fn(&Self));
}