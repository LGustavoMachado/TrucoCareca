use std::collections::HashMap;
use std::vec::Vec;

pub mod connection;
pub mod factories;
pub mod game_event;
pub mod models;
pub mod player;
pub mod state_machine;
pub mod states;

use connection::Connection;
use player::Player;

pub struct Game {
    output: Vec<(u32, String)>,
    list_players: HashMap<u32, (Connection, Player)>,
    seats: [Option<u32>; 4]
}

impl Game {
    pub fn new() -> Self {
        Self {
            output: vec![],
            seats: [None; 4],
            list_players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, id: u32, connection: Connection) -> Result<(), String> {
        if self.list_players.len() < 4 {
            self.list_players
                .insert(id, (connection, Player::new("player".to_string(), false)));
            println!("players: {}", self.list_players.len());
            Ok(())
        } else {
            Err(String::from("Atingiu o limite de jogadores"))
        }
    }

    pub fn is_full(&self) -> bool {
        self.list_players.len() == 4
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<(&mut Connection, &mut Player)> {
        if let Some((connection, player)) = self.list_players.get_mut(&id) {
            return Some((connection, player));
        }
        None
    }

    pub fn get_player_seat_index(&self, id: u32) -> Option<usize> {
        for (index, player_id) in self.seats.clone().iter().enumerate() {
            if player_id.is_none() {
                continue;
            }
            if player_id.unwrap() == id {
                return Some(index);
            }
        }
        None
    }

    pub fn are_players_ready(&self) -> bool {
        return self
            .list_players
            .values()
            .all(|(_, player)| player.is_ready());
    }

    pub fn output_mut(&mut self) -> &mut Vec<(u32, String)> {
        &mut self.output
    }

    pub fn player_output(&mut self, id: u32, message: String) {
        self.output.push((id, message));
    }

}

