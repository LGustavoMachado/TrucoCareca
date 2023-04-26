use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
// use futures_util::SinkExt;
// use tokio_tungstenite::tungstenite::Message;
// use crate::game::states::game_started_state::GameStartedState;
use crate::game::Game;

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
                    // If not empty send an error message
                    // let (connection, _) =  self.get_player_mut(id).unwrap();
                    // connection.sender.send(Message::Text(message)).await;
                    // We cannot send messages from here because we cannot make this method async, 
                    // we need to figure out a solution for this issue, otherwise the user wont 
                    // receive an error message
                    println!("ERROR SEAT IS OCCUPIED");
                }
            }
            GameEvent::LeaveSeatEvent(id) => {
                // Find the player seat
                // Remove from the seat
            }
            GameEvent::StartTheGameEvent() => {
                
            }
            _ => {}
        }

        None
    }
}
