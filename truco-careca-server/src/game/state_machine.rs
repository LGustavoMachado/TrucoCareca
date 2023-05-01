use crate::game::Game;

pub trait GameState: Send {
    fn init(&self, game: &mut Game);
    fn update(&self, game: &mut Game, time: f32) -> Option<Box<dyn GameState>>;
    fn state_out(&self, game: &Game) -> String;
}

pub struct StateMachine {
    state: Box<dyn GameState>, // Me da qualquer implementacao de GameState
}

impl StateMachine {
    pub fn new(initial_state: Box<dyn GameState>) -> Self {
        Self {
            state: initial_state,
        }
    }

    pub fn update(&mut self, game: &mut Game, time: f32) {
        if let Some(state) = self.state.update(game, time) {
            state.init(game);
            self.state = state;
        }
    }

    pub fn state_out(&self, game: &Game) -> String {
        self.state.state_out(game)
    }
}
