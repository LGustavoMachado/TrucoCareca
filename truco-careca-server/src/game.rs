pub mod connection;

use connection::Connection;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

pub struct Game {
    list_players: Vec<Connection>
}

impl Game {

    pub fn new() -> Self {
        Self { list_players: vec![]  }
    }

    pub fn add_player(&mut self, connection: Connection) -> Result<(), String> {
        if self.list_players.len() < 4 {
            self.list_players.push(connection);
            println!("players: {}", self.list_players.len());
            Ok(())
        } else {
            Err(String::from("Atingiu o limite de jogadores"))
        }
    }

    pub fn change_player_name(&mut self, id: u32, name: String) {
        if let Some(player) = self.list_players.iter_mut().find(|p| p.id == id) {
            player.name = name;
        }        
    }

    pub async fn send_message(&mut self, id: u32, message: String) {
        if let Some(player) = self.list_players.iter().find(|p| p.id == id) {
            let name = player.name.clone();
            for player in self.list_players.iter_mut().filter(|p| p.id != id) {
                let chat_message = String::from("") + &name + ": " + &message;
                player.sender.send(Message::Text(chat_message)).await.expect("Error sending message");
            }  
        }              
    }
}
