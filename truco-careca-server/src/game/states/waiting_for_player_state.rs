use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::states::waiting_for_ready_state::WaitingForReadyState;

pub struct WaitingForPlayersState {
}

impl WaitingForPlayersState {
  pub fn new() -> Self {
    println!("WAITING FOR PLAYERS STATE");
    Self {}
  }
}

impl GameState for WaitingForPlayersState {
  fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
    let event = GameEvent::None;
    match event {
      GameEvent::PlayerJoined(id, conn) => {
        let res = game.add_player(id, conn);
        match res {
          Result::Ok(()) => {
            if game.is_full() { 
              return Some(Box::new(WaitingForReadyState::new())); 
            }
          },
          Result::Err(_) => { }
        }
      }
      _ => { }
    }
    None
  }
}