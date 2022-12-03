use std::io::Read;
use crate::models::enums::{DynamicRacingLine, DynamicRacingLineType, GameMode, GearboxAssist, NetworkGame, RuleSet, SafetyCarStatus, SessionLength, SessionType, Weather, ZoneFlag};
use crate::models::PacketHeader;
use byteorder::{ReadBytesExt, LittleEndian};
use num_traits::FromPrimitive;
use crate::models::traits::Packet;
use crate::event_system::{Signal, Receiver};
use crate::event_system;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MarshalZone {
    pub zone_start: f32,
    pub zone_flag: ZoneFlag,
}

impl MarshalZone {
    pub fn new<R: Read>(reader: &mut R) -> Result<MarshalZone, std::io::Error> {
        Ok(MarshalZone {
            zone_start: reader.read_f32::<LittleEndian>()?,
            zone_flag: ZoneFlag::from_i8(reader.read_i8()?).unwrap(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WeatherForecastSample {
    pub session_type: SessionType,
    pub time_offset: u8,
    pub weather: Weather,
    pub track_temperature: i8,
    pub track_temperature_change: i8,
    pub air_temperature: i8,
    pub air_temperature_change: i8,
    pub rain_percentage: u8,
}

impl WeatherForecastSample {
    pub fn new<R: Read>(reader: &mut R) -> Result<WeatherForecastSample, std::io::Error> {
        Ok(WeatherForecastSample {
            session_type: SessionType::from_u8(reader.read_u8()?).unwrap(),
            time_offset: reader.read_u8()?,
            weather: Weather::from_u8(reader.read_u8()?).unwrap(),
            track_temperature: reader.read_i8()?,
            track_temperature_change: reader.read_i8()?,
            air_temperature: reader.read_i8()?,
            air_temperature_change: reader.read_i8()?,
            rain_percentage: reader.read_u8()?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionDataPacket {
    pub header: PacketHeader,
    pub weather: u8,
    pub track_temperature: i8,
    pub air_temperature: i8,
    pub total_laps: u8,
    pub track_length: u16,
    pub session_type: SessionType,

    pub track_id: i8,
    pub formula: u8,

    pub session_time_left: u16,
    pub session_duration: u16,
    pub pit_speed_limit: u8,
    pub game_paused: u8,
    pub is_spectating: u8,
    pub spectator_car_index: u8,
    pub sli_pro_native_support: u8,
    pub num_marshal_zones: u8,
    pub marshal_zones: Vec<MarshalZone>,
    pub safety_car_status: SafetyCarStatus,
    pub network_game: NetworkGame,
    pub num_weather_forecast_samples: u8,
    pub weather_forecast_samples: Vec<WeatherForecastSample>,
    pub forecast_accuracy: u8,
    pub ai_difficulty: u8,
    pub season_link_identifier: u32,
    pub weekend_link_identifier: u32,
    pub session_link_identifier: u32,
    pub pit_stop_window_ideal_lap: u8,
    pub pit_stop_window_latest_lap: u8,
    pub pit_stop_rejoin_position: u8,
    pub steering_assist: bool,
    pub braking_assist: bool,
    pub gearbox_assist: GearboxAssist,
    pub pit_assist: bool,
    pub pit_release_assist: bool,
    pub ers_assist: bool,
    pub drs_assist: bool,
    pub dynamic_racing_line: DynamicRacingLine,
    pub dynamic_racing_line_type: DynamicRacingLineType,
    pub game_mode: GameMode,
    pub rule_set: RuleSet,
    pub time_of_day: u32,
    pub session_length: SessionLength,
}

event_system::signal!(SessionSignal<SessionReceiver, SessionDataPacket> = []);

impl Packet for SessionDataPacket {
    const PACKET_ID: u8 = 1;
    const PACKET_SIZE: usize = 632;

    fn new<R: Read>(reader: &mut R) -> Result<SessionDataPacket, std::io::Error> {
        let header = PacketHeader::new(reader)?;
        let weather = reader.read_u8()?;
        let track_temperature = reader.read_i8()?;
        let air_temperature = reader.read_i8()?;
        let total_laps = reader.read_u8()?;
        let track_length = reader.read_u16::<LittleEndian>()?;
        let session_type = SessionType::from_u8(reader.read_u8()?).unwrap();
        let track_id = reader.read_i8()?;
        let formula = reader.read_u8()?;
        let session_time_left = reader.read_u16::<LittleEndian>()?;
        let session_duration = reader.read_u16::<LittleEndian>()?;
        let pit_speed_limit = reader.read_u8()?;
        let game_paused = reader.read_u8()?;
        let is_spectating = reader.read_u8()?;
        let spectator_car_index = reader.read_u8()?;
        let sli_pro_native_support = reader.read_u8()?;
        let num_marshal_zones = reader.read_u8()?;
        let mut marshal_zones = Vec::new();
        for _ in 0..21 {
            marshal_zones.push(MarshalZone::new(reader)?);
        }
        let safety_car_status = SafetyCarStatus::from_u8(reader.read_u8()?).unwrap();
        let network_game = NetworkGame::from_u8(reader.read_u8()?).unwrap();
        let num_weather_forecast_samples = reader.read_u8()?;
        let mut weather_forecast_samples = Vec::new();
        for _ in 0..56 {
            weather_forecast_samples.push(WeatherForecastSample::new(reader)?);
        }
        let forecast_accuracy = reader.read_u8()?;
        let ai_difficulty = reader.read_u8()?;
        let season_link_identifier = reader.read_u32::<LittleEndian>()?;
        let weekend_link_identifier = reader.read_u32::<LittleEndian>()?;
        let session_link_identifier = reader.read_u32::<LittleEndian>()?;
        let pit_stop_window_ideal_lap = reader.read_u8()?;
        let pit_stop_window_latest_lap = reader.read_u8()?;
        let pit_stop_rejoin_position = reader.read_u8()?;
        let steering_assist = reader.read_u8()? != 0;
        let braking_assist = reader.read_u8()? != 0;
        let gearbox_assist = GearboxAssist::from_u8(reader.read_u8()?).unwrap();
        let pit_assist = reader.read_u8()? != 0;
        let pit_release_assist = reader.read_u8()? != 0;
        let ers_assist = reader.read_u8()? != 0;
        let drs_assist = reader.read_u8()? != 0;
        let dynamic_racing_line = DynamicRacingLine::from_u8(reader.read_u8()?).unwrap();
        let dynamic_racing_line_type = DynamicRacingLineType::from_u8(reader.read_u8()?).unwrap();
        let game_mode = GameMode::from_u8(reader.read_u8()?).unwrap();
        let rule_set = RuleSet::from_u8(reader.read_u8()?).unwrap();
        let time_of_day = reader.read_u32::<LittleEndian>()?;
        let session_length = SessionLength::from_u8(reader.read_u8()?).unwrap();

        Ok(SessionDataPacket {
            header,
            weather,
            track_temperature,
            air_temperature,
            total_laps,
            track_length,
            session_type,
            track_id,
            formula,
            session_time_left,
            session_duration,
            pit_speed_limit,
            game_paused,
            is_spectating,
            spectator_car_index,
            sli_pro_native_support,
            num_marshal_zones,
            marshal_zones,
            safety_car_status,
            network_game,
            num_weather_forecast_samples,
            weather_forecast_samples,
            forecast_accuracy,
            ai_difficulty,
            season_link_identifier,
            weekend_link_identifier,
            session_link_identifier,
            pit_stop_window_ideal_lap,
            pit_stop_window_latest_lap,
            pit_stop_rejoin_position,
            steering_assist,
            braking_assist,
            gearbox_assist,
            pit_assist,
            pit_release_assist,
            ers_assist,
            drs_assist,
            dynamic_racing_line,
            dynamic_racing_line_type,
            game_mode,
            rule_set,
            time_of_day,
            session_length,
        })
    }

    event_system::signal_fns!();
}