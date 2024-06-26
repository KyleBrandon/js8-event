use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::event::Event;
use crate::parse_error::ParseError;
use super::message_type::MessageType;

#[derive(Debug, Serialize, Deserialize)]
pub struct RxSpot {
    #[serde(rename="CALL")]
    call: String,
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="GRID")]
    grid: String,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SNR")]
    snr: i8,
    #[serde(rename="_ID")]
    id: i64,
}

impl RxSpot {
    fn message(&self) -> String {
        format!("{}:{}:{}", self.call, self.dial, self.freq)
    }
}

impl TryFrom<Event> for RxSpot {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<RxSpot, Self::Error> {
        if *e.message_type() != MessageType::RxSpot {
            return Err(ParseError::InvalidMessageType);
        }

        let rx_spot: RxSpot = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(rx_spot)
    }

}


impl Display for RxSpot {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

