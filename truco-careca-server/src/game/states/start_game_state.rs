use crate::game::Game;
use crate::game::models::types::GameMode;
use crate::game::state_machine::GameState;
use crate::game::states::start_round_state::StartRoundState;

pub struct StartGameState {
}

impl StartGameState {
  pub fn new() -> Self {
    println!("GAME STARTED STATE");
    Self {}
  }
}

impl GameState for StartGameState {
  fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
    game.score = (0,0);
    game.dealer = 0;
    game.mode = GameMode::Normal;

    return Some(Box::new(StartRoundState::new())); 
  }
}