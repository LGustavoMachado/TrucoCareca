use crate::Connection;

pub enum GameEvent {
    PlayerJoined(u32, Connection),
    PlayerReady(u32, String),
}
