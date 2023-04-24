use crate::game::Game;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::states::game_started_state::GameStartedState;

pub struct WaitingForReadyState {
}

impl WaitingForReadyState {
  pub fn new() -> Self {
    println!("WAITING FOR READY STATE");
    Self {}
  }
}

impl GameState for WaitingForReadyState {
  fn update(&self, game: &mut Game, event: GameEvent) -> Option<Box<dyn GameState>> {
    match event {
      GameEvent::PlayerReady(id, name) => {
        let mut player = game.get_player_mut(id);
        player.name = name;
        player.is_ready = true;
      }
      _ => { }
    }
    // Verifica se todos os player estao ready
    if game.get_players().values().all(|(_, player)| player.is_ready) {
      return Some(Box::new(GameStartedState::new())); 
    }
    None
  }
}