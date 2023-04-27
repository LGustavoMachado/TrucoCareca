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

pub enum TurnResult {
    Winner(usize),
    Draw
}

use self::models::types::Card;
use self::models::types::Rank;
use self::models::types::Suit;
use self::models::deck_of_cards::DeckOfCards;
use self::models::types::GameMode;

pub struct Game {
    output: Vec<(u32, String)>,
    list_players: HashMap<u32, (Connection, Player)>,
    seats: [Option<u32>; 4],
    deck: DeckOfCards,
    pub head: usize, 
    pub turn: usize,
    pub score: (u8, u8),
    pub dealer: usize,
    pub turn_results: Vec<TurnResult>,
    pub table_cards: Vec<Card>,
    pub hands: [Vec<Card>; 4],
    pub round_value: u8,
    pub gave_up_players: Vec<usize>,
    pub manilha: Card,
    pub mode: GameMode,
}

impl Game {

    pub fn new() -> Self {
        let hand: Vec<Card> = Vec::new();
        Self {
            output: vec![],
            seats: [None; 4],
            list_players: HashMap::new(),
            deck: DeckOfCards::new(Vec::new()),
            head: 0,
            turn: 0,
            score: (0, 0),
            dealer: 0,
            turn_results: Vec::new(),
            table_cards: Vec::new(),
            hands: [hand.clone(), hand.clone(), hand.clone(), hand.clone()],
            round_value: 1,
            gave_up_players: Vec::new(),
            manilha: Card::new(Rank::Ace, Suit::Spades),
            mode: GameMode::Normal,
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

