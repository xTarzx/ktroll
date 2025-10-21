
use bincode::{Decode, Encode};

#[derive(Debug, Decode, Encode)]
pub enum KeyEventType {
    KeyPress,
    KeyRelease
}

#[derive(Debug, Decode, Encode)]
pub struct KeyEvent {
    pub event_type: KeyEventType,
    pub key: String,
}


