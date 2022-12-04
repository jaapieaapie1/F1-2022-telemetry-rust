use std::io;
use std::net::UdpSocket;
use crate::models::*;
use bytebuffer::ByteBuffer;
use crate::packets::Packets;
use crate::models::MotionPacket;
use crate::models::traits::Packet;

pub struct Server {
    pub stream: UdpSocket,
    address: &'static str,
}

impl Server {
    pub fn new_with_address(address: &'static str) -> io::Result<Server> {
        Ok(Server {
            stream: UdpSocket::bind(address)?,
            address,
        })
    }

    pub fn new() -> io::Result<Server> {
        Ok(Server {
            stream: UdpSocket::bind("0.0.0.0:25123")?,
            address: "0.0.0.0:25123",
        })
    }

    pub fn read_packet(&self) -> io::Result<Packets> {
        let mut buf: [u8; Self::calculate_size(PacketHeader::PACKET_SIZE)] = [0; Self::calculate_size(PacketHeader::PACKET_SIZE)];
        self.stream.peek(&mut buf)?;

        let header = PacketHeader::new(&mut ByteBuffer::from_bytes(&buf))?;

        Ok(
            match header.packet_id {
                MotionPacket::PACKET_ID => {
                    Packets::Motion(self.read_packet_dynamically::<MotionPacket>()?)
                },
                SessionDataPacket::PACKET_ID => {
                    Packets::Session(self.read_packet_dynamically::<SessionDataPacket>()?)
                },
                PacketLapData::PACKET_ID => {
                    Packets::LapData(self.read_packet_dynamically::<PacketLapData>()?)
                },
                PacketEventData::PACKET_ID => {
                    Packets::Event(self.read_packet_dynamically::<PacketEventData>()?)
                },
                ParticipantPacket::PACKET_ID => {
                    Packets::Participants(self.read_packet_dynamically::<ParticipantPacket>()?)
                },
                CarSetupPacket::PACKET_ID => {
                    Packets::CarSetups(self.read_packet_dynamically::<CarSetupPacket>()?)
                },
                CarTelemetryPacket::PACKET_ID => {
                    Packets::CarTelemetry(self.read_packet_dynamically::<CarTelemetryPacket>()?)
                },
                PacketCarStatus::PACKET_ID => {
                    Packets::CarStatus(self.read_packet_dynamically::<PacketCarStatus>()?)
                },
                PacketFinalClassificationData::PACKET_ID => {
                    Packets::FinalClassification(self.read_packet_dynamically::<PacketFinalClassificationData>()?)
                },
                PacketLobbyInfo::PACKET_ID => {
                    Packets::LobbyInfo(self.read_packet_dynamically::<PacketLobbyInfo>()?)
                },
                PacketCarDamage::PACKET_ID => {
                    Packets::CarDamage(self.read_packet_dynamically::<PacketCarDamage>()?)
                },
                PacketSessionHistory::PACKET_ID => {
                    Packets::SessionHistory(self.read_packet_dynamically::<PacketSessionHistory>()?)
                },
                _ => Packets::Unknown,
            }
        )
    }

    pub fn start(&self) {
        println!("Listening on {}", self.address);
        loop {
            let packet = self.read_packet();
            match packet {
                Ok(packet) => packet.emit_packet(),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    fn read_packet_dynamically<P: Packet>(&self) -> io::Result<P> {
        let mut buf: Vec<u8> = vec![0; Self::calculate_size(P::get_packet_size())];
        self.stream.recv(&mut buf)?;

        P::new(&mut ByteBuffer::from_bytes(&buf))
    }

    pub const fn calculate_size(size: usize) -> usize {
        if cfg!(windows) {
            return size + 2048
        }
        size
    }
}

