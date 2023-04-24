use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;

pub struct WaitingForReadyState {
}

impl WaitingForReadyState {
  pub fn new() -> Self {
    Self {}
  }
}

impl GameState for WaitingForReadyState {
  fn update(&self, _game: &mut Game, _event: GameEvent) -> Box<dyn GameState> {
    // if let GameEvent::PlayerReady(id, conn) = event {
    println!("WAITING FOR PLAYERS TO GET READY STATE");
    Box::new(WaitingForReadyState::new())
  }
}