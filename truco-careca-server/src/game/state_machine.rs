use crate::game::Game;
use crate::GameEvent;

pub trait GameState: Send {
  fn update(&self, game: &mut Game, event: GameEvent) -> Option<Box<dyn GameState>>;
}

pub struct StateMachine {
  state: Box<dyn GameState> // Me da qualquer implementacao de GameState
}

impl StateMachine {
  pub fn new(initial_state: Box<dyn GameState>) -> Self {
      Self { state: initial_state }
  }

  pub fn update(&mut self, game: &mut Game, event: GameEvent) {
    if let Some(state) = self.state.update(game, event) { 
      self.state = state;
    }
  }
}