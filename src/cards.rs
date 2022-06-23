//! Logic and helpful structs relating to cards and decks of cards.

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

/// Enum describing the rank of a card.
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
    /// Given a card's rank, return the short string representation of it.
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

/// Enum describing the suit of a card.
#[derive(EnumIter, EnumCountMacro, Copy, Clone, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    /// Given a card's suit, return the unicode symbol of that suit.
    fn unicode_representation(&self) -> &str {
        match self {
            Self::Club => "♣",
            Self::Diamond => "♦",
            Self::Heart => "♥",
            Self::Spade => "♠",
        }
    }
}

/// Object describing a playing card.
#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Representing the cards used in dealing and to give to players. Nobody owns it other than the game itself!
pub type Deck = Vec<Card>;

/// Represents the cards that a player owns.
pub type Hand = Vec<Card>;

impl fmt::Display for Card {
    /// Shows a card's rank and suit in a natural way.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.rank.simple_abbreviation(),
            self.suit.unicode_representation()
        )
    }
}

/// From the ranks and suits we described, gets the number of cards in a standard
/// deck (where there is one of each unique card present)
pub const STANDARD_DECK_COUNT: usize = Suit::COUNT * Rank::COUNT;

/// Creates a standard deck: an array of length `STANDARD_DECK_COUNT` containing one
/// of each unique card.
pub fn standard_deck() -> [Card; STANDARD_DECK_COUNT] {
    let mut card_collector: Vec<Card> = Vec::new();

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            card_collector.push(Card { rank, suit })
        }
    }

    // Guaranteed to be correct length of suits * ranks
    card_collector.try_into().unwrap()
}

/// Creates a deck of multiple standard decks.
///
/// # Arguments
///
/// * `num_decks` - The number of standard decks to use when creating the multideck.
///
/// # Examples
///
/// ```
/// use praeses_blackjack::cards::{create_multideck, STANDARD_DECK_COUNT};
///
/// let multideck = create_multideck(2);
/// assert_eq!(multideck.len(), STANDARD_DECK_COUNT * 2);
/// ```
pub fn create_multideck(num_decks: u32) -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    let standard_deck = standard_deck();

    for _ in 0..num_decks {
        deck.extend_from_slice(&standard_deck);
    }
    deck
}

/// Given a deck of cards, shuffles the deck efficiently.
///
/// # Arguments
///
/// * `deck: The deck to shuffle, as a list of cards.
pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}
