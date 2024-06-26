use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::event::Event;
use crate::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct RigPtt {
    #[serde(rename="PTT")]
    ptt: bool,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="_ID")]
    id: i64,
}


impl TryFrom<Event> for RigPtt {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<RigPtt, Self::Error> {
        if *e.message_type() != MessageType::RigPtt {
            return Err(ParseError::InvalidMessageType);
        }

        let rig_ptt: RigPtt = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(rig_ptt)
    }

}
