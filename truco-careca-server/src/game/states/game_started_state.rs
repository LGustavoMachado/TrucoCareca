use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;

pub struct GameStartedState {
}

impl GameStartedState {
  pub fn new() -> Self {
    println!("GAME STARTED STATE");
    Self {}
  }
}

impl GameState for GameStartedState {
  fn update(&self, _game: &mut Game, _event: GameEvent) -> Option<Box<dyn GameState>> {
    None
  }
}