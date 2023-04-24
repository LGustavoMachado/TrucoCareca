use crate::game::Game;
use crate::GameEvent;

pub trait GameState: Send {
  fn update(&self, game: &mut Game, event: GameEvent) -> Box<dyn GameState>;
}

pub struct StateMachine {
  state: Box<dyn GameState>
}

impl StateMachine {
  pub fn new(initial_state: Box<dyn GameState>) -> Self {
      Self { state: initial_state }
  }

  pub fn update(&mut self, game: &mut Game, event: GameEvent) {
      self.state = self.state.update(game, event);
  }
}