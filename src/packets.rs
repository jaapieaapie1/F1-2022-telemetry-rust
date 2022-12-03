use crate::models::{CarSetupPacket, CarTelemetryPacket, MotionPacket, PacketCarDamage, PacketCarStatus, PacketEventData, PacketFinalClassificationData, PacketLapData, PacketLobbyInfo, PacketSessionHistory, ParticipantPacket, SessionDataPacket};
use crate::models::traits::Packet;

pub enum Packets {
    Motion(MotionPacket),
    Session(SessionDataPacket),
    LapData(PacketLapData),
    Event(PacketEventData),
    Participants(ParticipantPacket),
    CarSetups(CarSetupPacket),
    CarTelemetry(CarTelemetryPacket),
    CarStatus(PacketCarStatus),
    FinalClassification(PacketFinalClassificationData),
    LobbyInfo(PacketLobbyInfo),
    CarDamage(PacketCarDamage),
    SessionHistory(PacketSessionHistory),
    Unknown,
}

impl Packets {
    pub fn emit_packet(self) {
        match self {
            Packets::Motion(packet) => packet.emit(),
            Packets::Session(packet) => packet.emit(),
            Packets::LapData(packet) => packet.emit(),
            Packets::Event(packet) => packet.emit(),
            Packets::Participants(packet) => packet.emit(),
            Packets::CarSetups(packet) => packet.emit(),
            Packets::CarTelemetry(packet) => packet.emit(),
            Packets::CarStatus(packet) => packet.emit(),
            Packets::FinalClassification(packet) => packet.emit(),
            Packets::LobbyInfo(packet) => packet.emit(),
            Packets::CarDamage(packet) => packet.emit(),
            Packets::SessionHistory(packet) => packet.emit(),
            Packets::Unknown => println!("Unknown packet"),
        }
    }
}