use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::event::Event;
use crate::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct StationStatus {
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SELECTED")]
    selected: String,
    #[serde(rename="SPEED")]
    speed: u8,
    #[serde(rename="_ID")]
    id: String,
}

impl TryFrom<Event> for StationStatus {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<StationStatus, Self::Error> {
        if *e.message_type() != MessageType::StationStatus {
            return Err(ParseError::InvalidMessageType);
        }

        let station_status: StationStatus = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(station_status)
    }

}
