use crate::game::models::deck_of_cards::DeckOfCards;
use crate::game::models::types::*;
use strum::IntoEnumIterator;

pub enum DeckType {
    Clean = 4,
    Dirty = 0,
}

pub fn create_deck(deck_type: DeckType) -> DeckOfCards {
    let mut cards = Vec::new();
    let skip = deck_type as usize;

    for rank in Rank::iter().skip(skip) {
        for suit in Suit::iter() {
            cards.push(Card::new(rank.clone(), suit.clone()));
        }
    }
    DeckOfCards::new(cards)
}

mod tests {
    use super::*;

    #[test]
    fn test_create_deck_clean() {
        let deck_type = DeckType::Clean;
        let deck = create_deck(deck_type);
        assert_eq!(deck.cards.len(), 24);
    }

    #[test]
    fn test_create_deck_dirty() {
        let deck_type = DeckType::Dirty;
        let deck = create_deck(deck_type);
        assert_eq!(deck.cards.len(), 40);
    }

    #[test]
    fn test_create_deck_with_rank() {
        let deck_type = DeckType::Dirty;
        let deck = create_deck(deck_type);
        let cards_with_rank = deck
            .cards
            .iter()
            .filter(|&card| card.rank == Rank::Ace)
            .collect::<Vec<_>>();
        assert_eq!(cards_with_rank.len(), 4);
    }

    #[test]
    fn test_create_deck_with_suit() {
        let deck_type = DeckType::Dirty;
        let deck = create_deck(deck_type);
        let cards_with_suit = deck
            .cards
            .iter()
            .filter(|&card| card.suit == Suit::Hearts)
            .collect::<Vec<_>>();
        assert_eq!(cards_with_suit.len(), 10);
    }
}
