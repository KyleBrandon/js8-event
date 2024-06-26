use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::event::Event;
use crate::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct RxActivity {
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SNR")]
    snr: i8,
    #[serde(rename="SPEED")]
    speed: u8,
    #[serde(rename="TDRIFT")]
    tdrift: f64,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="_ID")]
    id: i64,
}

impl RxActivity {
    fn message(&self) -> String {
        format!("{}:{}", self.dial, self.freq)
    }

}

impl TryFrom<Event> for RxActivity {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<RxActivity, Self::Error> {
        if *e.message_type() != MessageType::RxActivity {
            return Err(ParseError::InvalidMessageType);
        }

        let rx_activity: RxActivity = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(rx_activity)
    }

}

impl Display for RxActivity {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}



