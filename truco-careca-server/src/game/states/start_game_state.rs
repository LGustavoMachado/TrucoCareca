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

  fn init(&self, game: &mut Game) {
    let ids: Vec<u32> = game.get_players().keys().cloned().collect();
    for id in ids {
        game.output(id, "START GAME STATE".to_string());
    }
  }
  
  fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
    game.score = (0,0);
    game.dealer = 0;
    game.mode = GameMode::Normal;

    return Some(Box::new(StartRoundState::new())); 
  }

  fn state_out(&self, game: &Game) -> String { 
    "".to_string()
  }
}