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
                let seat = seat as usize;
                // Validate if the seat is empty
                let seat_free = game.seats[seat].is_none();

                if seat_free {
                    // Remove player from the seat if he has a seat already
                    for (index, player_id) in game.seats.clone().iter().enumerate() {
                        if player_id.is_none() {
                            continue;
                        }
                        if player_id.unwrap() == id {
                            game.seats[index] = None;
                        }
                    }
                    // add to seat
                    game.seats[seat] = Some(id);
                    println!("Player {} seated pos: {}", id, seat);
                } else {
                    // If not empty send an error message to player
                    game.player_output(id, String::from("SEAT IS OCCUPIED"));
                }
            }
            GameEvent::LeaveSeatEvent(_id) => {
                // Find the player seat
                // Remove from the seat
            }
            GameEvent::StartTheGameEvent() => {
                return Some(Box::new(GameStartedState::new()));
            }
            _ => {}
        }

        None
    }
}
