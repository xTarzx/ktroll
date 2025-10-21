
use bincode::{Decode, Encode};

#[derive(Debug, Decode, Encode)]
pub struct KeyEvent {
    pub event_type: String,
    pub key: String,
}


