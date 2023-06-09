use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::Game;
use super::start_game_state::StartGameState;

use serde_json::{json, Value};
use serde::Deserialize;
use queues::IsQueue;

pub struct PickUpSeatsState {}

impl PickUpSeatsState {
    pub fn new() -> Self {
        println!("PICK UP SEATS STATE");
        Self {}
    }
}

impl GameState for PickUpSeatsState {
    
    fn init(&self, game: &mut Game) {
        let ids: Vec<u32> = game.get_players().keys().cloned().collect();
        for id in ids {
            game.output(id, "PICK UP SEATS STATE".to_string());
        }
    }

    fn update(&self, game: &mut Game, _time: f32) -> Option<Box<dyn GameState>> {
        while let Ok(event) = game.inputs.remove() {
            match event {
                GameEvent::PickUpSeatEvent(id, seat) => {
                    if seat > 3 { return None; }

                    let seat = seat as usize;
                    let seat_free = game.seats[seat].is_none();

                    if seat_free {
                        if let Some(index) = game.get_player_seat_index(id) {
                            game.seats[index] = None;
                        }
                        game.seats[seat] = Some(id); // add to seat
                        game.output(id, format!("Player {} in seat {}", id, seat));
                    } else {
                        game.output(id, String::from("SEAT IS OCCUPIED"));
                    }
                }
                GameEvent::LeaveSeatEvent(id) => {
                    // Find the player seat and remove it
                    if let Some(index) = game.get_player_seat_index(id) {
                        game.seats[index] = None;
                        game.output(id, format!("Player left seat {}", index));
                    }
                }
                GameEvent::StartTheGameEvent => {
                    if game.seats.iter().all(Option::is_some) {
                        return Some(Box::new(StartGameState::new()));
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn state_out(&self, game: &Game, _player_id: u32) -> String {

        let mut seats: Vec<serde_json::Value> = vec![];

        for seat in game.seats() {
            match seat {
                Some(id) => {
                    let (_, player) = game.get_player(*id).unwrap();
                    seats.push(json!({
                        "id": id,
                        "name": player.name
                    }))
                }
                None => {
                    seats.push(json!(null))
                }
            }
        }

        json!({
            "state": {
                "name": "pick-up-seats",
                "body": {
                    "seats": seats
                }
            },
        }).to_string()
    }

}