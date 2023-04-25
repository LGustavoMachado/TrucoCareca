use crate::game::models::types::Card;
use rand::seq::SliceRandom;
use std::vec::Vec;

#[derive(Clone)]
pub struct DeckOfCards {
    pub cards: Vec<Card>,
}

impl DeckOfCards {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        self
    }

    fn cut(mut self, n_cards: u8) -> Self {
        let len = self.cards.len() as u8;
        let cut_position = (n_cards % len) as usize;
        let (left_half, right_half) = self.cards.split_at(cut_position);

        self.cards = [right_half, left_half].concat();
        self
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

mod tests {
    use crate::game::factories::deck_factory::{create_deck, DeckType}; // etc.

    #[test]
    fn test_shuffle_deck() {
        let deck_type = DeckType::Dirty;
        let new_deck = create_deck(deck_type);
        let shuffled = new_deck.clone().shuffle();

        assert_ne!(new_deck.cards, shuffled.cards);
    }

    #[test]
    fn test_cut_deck() {
        let deck_type = DeckType::Dirty;
        let new_deck = create_deck(deck_type);
        let cut = new_deck.clone().cut(10);

        assert_eq!(new_deck.cards[0], cut.cards[30]);
    }

    #[test]
    fn test_deal_card() {
        let deck_type = DeckType::Dirty;
        let mut new_deck = create_deck(deck_type);
        let last_card = new_deck.cards[39];
        let dealt_card = new_deck.deal();

        assert_eq!(last_card, dealt_card.unwrap());
    }
}
