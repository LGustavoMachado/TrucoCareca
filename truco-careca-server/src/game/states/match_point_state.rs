use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;

pub struct MatchPointState {
}

impl MatchPointState {
  pub fn new() -> Self {
    println!("PLAYER TURN STATE");
    Self {}
  }
}

impl GameState for MatchPointState {
  fn update(&self, game: &mut Game, _event: GameEvent) -> Option<Box<dyn GameState>> {
    None 
  }
}