use crate::game::Game;
use crate::game::state_machine::GameState;

pub struct PlayerTurnState {
}

impl PlayerTurnState {
  pub fn new() -> Self {
    println!("PLAYER TURN STATE");
    Self {}
  }
}

impl GameState for PlayerTurnState {

  fn init(&self, game: &mut Game) { }
  
  fn update(&self, _game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
    None
  }

  fn state_out(&self, game: &Game) -> String { 
    "".to_string()
  }
}