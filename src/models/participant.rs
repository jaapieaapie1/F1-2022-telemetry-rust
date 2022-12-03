use std::io::Read;
use crate::models::enums::Team;
use num_traits::FromPrimitive;
use byteorder::ReadBytesExt;
use crate::models::PacketHeader;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ParticipantData {
    pub ai_controlled: bool,
    pub driver_id: u8,
    pub network_id: u8,
    pub team: Team,
    pub my_team: bool,
    pub race_number: u8,
    pub nationality: u8,
    pub name: String,
    pub your_telemetry: bool,
}

impl ParticipantData {
    pub fn new<R: Read>(reader: &mut R) -> Result<ParticipantData, std::io::Error> {
        Ok(ParticipantData {
            ai_controlled: reader.read_u8()? == 1,
            driver_id: reader.read_u8()?,
            network_id: reader.read_u8()?,
            team: Team::from_u8(reader.read_u8()?).unwrap(),
            my_team: reader.read_u8()? == 1,
            race_number: reader.read_u8()?,
            nationality: reader.read_u8()?,
            name: {
                let mut name = String::new();
                for _ in 0..48 {
                    name.push(reader.read_u8()? as char);
                }
                name
            },
            your_telemetry: reader.read_u8()? == 1,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ParticipantPacket {
    pub header: PacketHeader,
    pub num_active_cars: u8,
    pub participants: Vec<ParticipantData>,
}

event_system::signal!(ParticipantSignal<ParticipantReceiver, ParticipantPacket> = []);

impl Packet for ParticipantPacket {

    const PACKET_ID: u8 = 4;
    const PACKET_SIZE: usize = 1257;

    fn new<R: Read>(reader: &mut R) -> Result<ParticipantPacket, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let num_active_cars = reader.read_u8()?;
        let mut participants = Vec::new();
        for _ in 0..22 {
            participants.push(ParticipantData::new(reader)?);
        }
        Ok(ParticipantPacket {
            header,
            num_active_cars,
            participants,
        })
    }

    event_system::signal_fns!();
}