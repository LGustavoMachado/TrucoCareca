use strum_macros::EnumIter;

pub enum GameMode {
    Normal,
    IronHands
}

#[derive(Copy, Clone, EnumIter, Debug, PartialEq)]
pub enum Suit {
    Diamonds,
    Spades,
    Hearts,
    Clubs,
}

#[derive(Copy, Clone, EnumIter, Debug, PartialEq)]
pub enum Rank {
    Four = 1,
    Five = 2,
    Six = 3,
    Seven = 4,
    Queen = 5,
    Jack = 6,
    King = 7,
    Ace = 8,
    Two = 9,
    Three = 10,
}

pub enum Manilha {
    Picafumo = 11,
    Espadilha = 12,
    Copeta = 13,
    Zap = 14,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}
