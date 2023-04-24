use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;
use std::collections::HashMap;

pub mod player;
pub mod state_machine;
pub mod connection;
pub mod game_event;
pub mod states;

use player::Player;
use connection::Connection;

pub struct Game {
    list_players: HashMap<u32, (Connection, Player)>
}

impl Game {

    pub fn new() -> Self {
        Self { list_players: HashMap::new() } // k = id, v = tuple => con + player
    }

    pub fn add_player(&mut self, id: u32, connection: Connection) -> Result<bool, String> {
        if self.list_players.len() < 4 {
            self.list_players.insert(
                id,
                (connection, Player::new("player".to_string(), false))
            );
            println!("players: {}", self.list_players.len());
            Ok(self.list_players.len() == 4)
        } else {
            Err(String::from("Atingiu o limite de jogadores"))
        }
    }

    pub fn change_player_name(&mut self, id: u32, name: String) {
        if let Some((_, player)) = self.list_players.get_mut(&id) {
            player.name = name;
        }        
    }

    pub async fn send_message(&mut self, sender_id: u32, message: String) {
        if let Some((_, sender_player)) = self.list_players.get(&sender_id) {
            let name = sender_player.name.clone();
            for (id, (conn, _)) in self.list_players.iter_mut() {
                if *id == sender_id {
                    continue;
                }

                let chat_message = String::from("") + &name + ": " + &message;
                conn.sender.send(Message::Text(chat_message)).await.expect("Error sending message");
            }  
        }           
    }   
}
