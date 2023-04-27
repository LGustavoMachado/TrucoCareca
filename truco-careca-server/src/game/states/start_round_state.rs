use crate::game::Game;
use crate::game::models::types::GameMode;
use crate::game::game_event::GameEvent;
use crate::game::state_machine::GameState;
use crate::game::factories::deck_factory::{create_deck, DeckType};
use crate::game::states::player_turn_state::PlayerTurnState;
use crate::game::states::match_point_state::MatchPointState;

pub struct StartRoundState {
}

impl StartRoundState {
  pub fn new() -> Self {
    println!("START ROUND STATE");
    Self {}
  }
}

impl GameState for StartRoundState {
  fn update(&self, game: &mut Game, _event: GameEvent) -> Option<Box<dyn GameState>> {
    game.turn = 0;
    game.table_cards = Vec::new();
    game.hands = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    game.round_value = 1;
    game.gave_up_players = Vec::new();
    game.deck = create_deck(DeckType::Dirty);
    game.deck.shuffle();
    game.deck.cut(20);
    game.head = game.turn + 1;

    if is_iron_hands(game.score) {
      game.mode = GameMode::IronHands;
    }

    for _ in 0..2 {
      for i in 0..3 {
        let card = game.deck.deal().unwrap();
        game.hands[i].push(card);
      };
    }

    game.manilha = game.deck.deal().unwrap();

    if is_match_point(game.score) {
      return Some(Box::new(MatchPointState::new()));
    }

    return Some(Box::new(PlayerTurnState::new()))
  }
}

fn is_match_point(score: (u8, u8)) -> bool {
  let mut count = 0;
  if score.0 == 11 { count += 1 };
  if score.1 == 11 { count += 1 };
  count == 1
}

fn is_iron_hands(score: (u8, u8)) -> bool {
  let mut count = 0;
  if score.0 == 11 { count += 1 };
  if score.1 == 11 { count += 1 };
  count == 2
}