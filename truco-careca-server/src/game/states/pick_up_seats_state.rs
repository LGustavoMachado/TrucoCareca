use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::Game;

use super::game_started_state::GameStartedState;

pub struct PickUpSeatsState {}

impl PickUpSeatsState {
    pub fn new() -> Self {
        println!("PICK UP SEATS STATE");
        Self {}
    }
}

impl GameState for PickUpSeatsState {
    fn update(&self, game: &mut Game, event: GameEvent) -> Option<Box<dyn GameState>> {
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
                    game.player_output(id, format!("Player {} in seat {}", id, seat));
                } else {
                    game.player_output(id, String::from("SEAT IS OCCUPIED"));
                }
            }
            GameEvent::LeaveSeatEvent(id) => {
                // Find the player seat and remove it
                if let Some(index) = game.get_player_seat_index(id) {
                    game.seats[index] = None;
                    game.player_output(id, format!("Player left seat {}", index));
                }
            }
            GameEvent::StartTheGameEvent => {
                if game.seats.iter().all(Option::is_some) {
                    return Some(Box::new(GameStartedState::new()));
                }
            }
            _ => {}
        }

        if game.seats.iter().all(Option::is_some) {
            println!("Seats are all occupied")
        }

        None
    }
}
