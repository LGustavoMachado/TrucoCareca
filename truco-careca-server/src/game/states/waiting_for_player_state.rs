use crate::game::Game;
use crate::game::state_machine::GameState;
use crate::game::states::waiting_for_ready_state::WaitingForReadyState;

use serde_json::{json};

pub struct WaitingForPlayersState {
}

impl WaitingForPlayersState {
  pub fn new() -> Self {
    println!("WAITING FOR PLAYERS STATE");
    Self {}
  }
}

impl GameState for WaitingForPlayersState {

  fn init(&self, game: &mut Game) { }

  fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
    if game.is_full() {
      return Some(Box::new(WaitingForReadyState::new()));
    }
    None
  }

  fn state_out(&self, game: &Game, player_id: u32) -> String {
    json!({
      "state": {
          "name": "waiting-for-ready"
      }
    }).to_string()
  }
}