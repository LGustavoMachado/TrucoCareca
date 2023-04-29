use crate::TrucoCommand;

#[derive(Clone)]
pub enum GameEvent {
  PlayerReady(u32, String),
  PickUpSeatEvent(u32, u32),
  LeaveSeatEvent(u32),
  StartTheGameEvent,
  None,
}

pub fn get_event_from_command(id: u32, command: TrucoCommand) -> Option<GameEvent> {
    match command.name.as_str() {
        "player-ready" => {
            let name = command.body.get("name").unwrap();
            return Some(GameEvent::PlayerReady(id, name.to_string()));
        }
        "pick-up-seat" => {
            let seat = command.body.get("seat").unwrap();
            let seat = seat.as_u64().unwrap();
            return Some(GameEvent::PickUpSeatEvent(id, seat as u32));
        }
        "leave-seat" => {
            return Some(GameEvent::LeaveSeatEvent(id));
        }
        "start-the-game" => {
            return Some(GameEvent::StartTheGameEvent);
        }
        _ => None,
    }
}