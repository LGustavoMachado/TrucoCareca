use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::states::waiting_for_ready_state::WaitingForReadyState;
use crate::game::Game;

pub struct WaitingForPlayersState {}

impl WaitingForPlayersState {
    pub fn new() -> Self {
        println!("WAITING FOR PLAYERS STATE");
        Self {}
    }
}

impl GameState for WaitingForPlayersState {
    fn update(&self, game: &mut Game, event: GameEvent) -> Option<Box<dyn GameState>> {
        match event {
            GameEvent::PlayerJoined(id, conn) => {
                let res = game.add_player(id, conn);
                match res {
                    Result::Ok(()) => {
                        if game.is_full() {
                            return Some(Box::new(WaitingForReadyState::new()));
                        }
                    }
                    Result::Err(_) => {}
                }
            }
            _ => {}
        }
        None
    }
}
