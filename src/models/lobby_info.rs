use std::io::Read;
use crate::models::PacketHeader;
use byteorder::ReadBytesExt;
use num_traits::FromPrimitive;
use crate::models::enums::{ReadyStatus, Team};
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LobbyInfo {
    pub ai_controlled: bool,
    pub team: Team,
    pub nationality: u8,
    pub name: String,
    pub car_number: u8,
    pub ready_status: ReadyStatus,
}

impl LobbyInfo {
    pub fn new<R: Read>(reader: &mut R) -> Result<LobbyInfo, std::io::Error> {
        Ok(LobbyInfo {
            ai_controlled: reader.read_u8()? == 1,
            team: Team::from_u8(reader.read_u8()?).unwrap(),
            nationality: reader.read_u8()?,
            name: {
                let mut v = Vec::new();
                for _ in 0..48 {
                    v.push(reader.read_u8()?);
                }
                String::from_utf8(v).unwrap()
            },
            car_number: reader.read_u8()?,
            ready_status: ReadyStatus::from_u8(reader.read_u8()?).unwrap(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PacketLobbyInfo {
    pub header: PacketHeader,
    pub num_lobbies: u8,
    pub lobbies: Vec<LobbyInfo>,
}

event_system::signal!(LobbyInfoSignal<LobbyInfoReceiver, PacketLobbyInfo> = []);

impl Packet for PacketLobbyInfo {
    const PACKET_ID: u8 = 9;
    const PACKET_SIZE: usize = 1191;

    fn new<R: Read>(reader: &mut R) -> Result<PacketLobbyInfo, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let num_lobbies = reader.read_u8()?;
        let mut lobbies = Vec::new();
        for _ in 0..22 {
            lobbies.push(LobbyInfo::new(reader)?);
        }
        Ok(PacketLobbyInfo {
            header,
            num_lobbies,
            lobbies,
        })
    }

    event_system::signal_fns!();
}