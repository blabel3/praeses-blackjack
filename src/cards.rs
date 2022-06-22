//! This module houses logic relating to cards and card value

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(EnumIter, EnumCountMacro, Copy, Clone, Debug)]
pub enum Rank {
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
    King,
}

impl Rank {
    const fn simple_abbreviation(&self) -> &str {
        match self {
            Self::Ace => "A",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
        }
    }
}

#[derive(EnumIter, EnumCountMacro, Copy, Clone, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
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

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.rank.simple_abbreviation(),
            self.suit.unicode_representation()
        )
    }
}

pub const STANDARD_DECK_COUNT: usize = Suit::COUNT * Rank::COUNT;

pub fn standard_deck() -> [Card; Suit::COUNT * Rank::COUNT] {
    let mut card_collector: Vec<Card> = Vec::new();

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            card_collector.push(Card { rank, suit })
        }
    }

    // Guaranteed to be correct length of suits * ranks
    card_collector.try_into().unwrap()
}

pub fn get_shuffled_deck(num_decks: u32) -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    let standard_deck = standard_deck();

    for _ in 0..num_decks {
        deck.extend_from_slice(&standard_deck);
    }
    deck.shuffle(&mut thread_rng());
    deck
}
