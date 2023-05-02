use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::states::pick_up_seats_state::PickUpSeatsState;
use crate::game::Game;

use serde_json::{json};
use queues::IsQueue;

pub struct WaitingForReadyState {}

impl WaitingForReadyState {
    pub fn new() -> Self {
        Self {}
    }
}


impl GameState for WaitingForReadyState {

    fn init(&self, game: &mut Game) {}

    fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
        while let Ok(event) = game.inputs.remove() {
            match event {
                GameEvent::PlayerReady(id, name) => {
                    if let Some((_, player)) = game.get_player_mut(id) {
                        player.update(name.clone(), true);
                        println!("Player {} id: {} is ready", name, id);
                        game.output(id, "Ready!".to_string());
                    }
                }
                _ => {}
            }
            // Verifica se todos os player estao ready
            if game.are_players_ready() {
                return Some(Box::new(PickUpSeatsState::new()));
            }
        }
        None
    }

    fn state_out(&self, game: &Game, player_id: u32) -> String {
        let (_, player) = game.get_player(player_id).unwrap();
        json!({
            "state": {
                "name": "waiting-for-ready",
                "body": {
                    "player_ready": player.is_ready()
                }
            },
        }).to_string()
    }
}
