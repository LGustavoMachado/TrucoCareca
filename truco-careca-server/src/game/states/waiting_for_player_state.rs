use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::states::waiting_for_ready_state::WaitingForReadyState;

pub struct WaitingForPlayersState {
}

impl WaitingForPlayersState {
  pub fn new() -> Self {
    Self {}
  }
}

impl GameState for WaitingForPlayersState {
  fn update(&self, game: &mut Game, event: GameEvent) -> Box<dyn GameState> {
    println!("WAITING FOR PLAYERS STATE");
    if let GameEvent::PlayerJoined(id, conn) = event {
      match game.add_player(id, conn){
        Ok(is_game_full) => {
          if is_game_full {
            return Box::new(WaitingForReadyState::new());
          }
        },
        Err(error) => { println!("Error: {}", error) }
      }
    }

    Box::new(WaitingForPlayersState::new())   
  }
}