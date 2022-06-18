use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Debug)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Rank {
    const fn numeric_value(&self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 10,
            Self::Queen => 10,
            Self::King => 10,
        }
    }
}

#[derive(EnumIter, Copy, Clone, Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Suit {
    fn unicode_representation(&self) -> &str {
        match self {
            Self::Club => "♣",
            Self::Diamond => "♦",
            Self::Heart => "♥",
            Self::Spade => "♠",
        }
    }
}

#[derive(Debug)]
pub struct Card{
    rank: Rank,
    suit: Suit
}

impl Card {
    pub fn standard_deck() -> [Card; 52] {
        let mut card_collector: Vec<Card> = Vec::new();

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                card_collector.push(Card {
                    rank,
                    suit
                })
            }
        }

        card_collector.try_into().unwrap()
    }
}
