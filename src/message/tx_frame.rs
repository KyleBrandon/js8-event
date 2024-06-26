use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::event::Event;
use crate::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct TxFrame {
    #[serde(rename="TONES")]
    tones: Vec<u8>,
    #[serde(rename="_ID")]
    id: i64,
}

impl TryFrom<Event> for TxFrame {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<TxFrame, Self::Error> {
        if *e.message_type() != MessageType::TxFrame {
            return Err(ParseError::InvalidMessageType);
        }

        let tx_frame: TxFrame = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(tx_frame)
    }

}
