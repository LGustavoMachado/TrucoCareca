use crate::game::Game;
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
    if game.is_full() {
      return Some(Box::new(WaitingForReadyState::new()));
    }
    None
  }
}