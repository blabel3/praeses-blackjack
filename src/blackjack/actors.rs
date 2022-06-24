//! Logic for Blackjack actors: what behaviors everyone involved in a game of blackjack
//! (dealers, players, etc) needs to know how to do.
//! How they decide what to do and
//! what happens when they make their action are here. Using this,
//! you can create players that behave differently but all act
//! within the allowed moves in Blackjack.

pub mod dealers;
pub mod players;

use crate::cards;

/// Supported player actions.
#[derive(Debug, PartialEq)]
pub enum Action {
    /// Adds a card from the deck to your hand
    Hit,
    /// Keep the cards in your hand and pass to the next player
    Stand,
}

impl Action {
    /// Provides a default prompt for actions in the commandline.
    pub const ACTION_PROMPT: &'static str = "Hit (h) or Stand (s)?";

    /// From an input string, return an action if there is an appropriate match found.
    /// If not, return an error.
    pub fn parse_from_string(input: &str) -> Result<Self, &'static str> {
        let input = &input.trim().to_lowercase()[..];
        match input {
            "hit" | "h" => Ok(Self::Hit),
            "stand" | "s" => Ok(Self::Stand),
            _ => Err("Invalid action input"),
        }
    }
}

pub trait Actor {
    /// Get a reference to the player's hand, all the cards they have.
    fn get_hand(&self) -> &cards::Hand;

    /// Syntactic sugar for getting a slice from their hand.
    /// Equivalent to `&self.get_hand()[..]`
    fn get_hand_slice(&self) -> &[cards::Card] {
        self.get_hand().as_slice()
    }

    /// Display the user's current hand in a natural way.
    fn show_hand(&self) -> ();

    // Can probably turn new and this into a macro maybe?
    /// Add a card given in the argument to a player's hand.
    fn recieve_card(&mut self, card: cards::Card) -> ();
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn add_card_to_hand<T: Actor>(mut actor: T) {
        assert_eq!(0, actor.get_hand().len());
        actor.recieve_card(cards::Card {
            rank: cards::Rank::Ace,
            suit: cards::Suit::Spade,
        });

        assert_eq!(1, actor.get_hand().len());
    }

    pub fn create_card_from_value(value: u32) -> cards::Card {
        match value {
            1 => cards::Card {
                rank: cards::Rank::Ace,
                suit: cards::Suit::Spade,
            },
            2 => cards::Card {
                rank: cards::Rank::Two,
                suit: cards::Suit::Spade,
            },
            3 => cards::Card {
                rank: cards::Rank::Three,
                suit: cards::Suit::Spade,
            },
            4 => cards::Card {
                rank: cards::Rank::Four,
                suit: cards::Suit::Spade,
            },
            5 => cards::Card {
                rank: cards::Rank::Five,
                suit: cards::Suit::Spade,
            },
            6 => cards::Card {
                rank: cards::Rank::Six,
                suit: cards::Suit::Spade,
            },
            7 => cards::Card {
                rank: cards::Rank::Seven,
                suit: cards::Suit::Spade,
            },
            8 => cards::Card {
                rank: cards::Rank::Eight,
                suit: cards::Suit::Spade,
            },
            9 => cards::Card {
                rank: cards::Rank::Nine,
                suit: cards::Suit::Spade,
            },
            10 => cards::Card {
                rank: cards::Rank::Ten,
                suit: cards::Suit::Spade,
            },
            _ => panic!("Tried to make a card of an invalid value in tests."),
        }
    }

    #[test]
    fn parses_action_from_string() {
        assert_eq!(Action::parse_from_string("hit").unwrap(), Action::Hit);
        assert_eq!(Action::parse_from_string("h").unwrap(), Action::Hit);
        assert_eq!(Action::parse_from_string(" hit").unwrap(), Action::Hit);
        assert_eq!(Action::parse_from_string("hit ").unwrap(), Action::Hit);
        assert_eq!(Action::parse_from_string("Hit").unwrap(), Action::Hit);
        assert_eq!(Action::parse_from_string("HIT").unwrap(), Action::Hit);

        assert_eq!(Action::parse_from_string("stand").unwrap(), Action::Stand);
        assert_eq!(Action::parse_from_string("s").unwrap(), Action::Stand);
        assert_eq!(Action::parse_from_string(" stand").unwrap(), Action::Stand);
        assert_eq!(Action::parse_from_string("stand ").unwrap(), Action::Stand);
        assert_eq!(Action::parse_from_string("Stand").unwrap(), Action::Stand);
        assert_eq!(Action::parse_from_string("STAND").unwrap(), Action::Stand);

        assert!(Action::parse_from_string("shmit").is_err());
        assert!(Action::parse_from_string("stund").is_err());
        assert!(Action::parse_from_string("hoot").is_err());
        assert!(Action::parse_from_string("ham").is_err());
        assert!(Action::parse_from_string("praeses").is_err());
        assert!(Action::parse_from_string("blake").is_err());
    }
}
