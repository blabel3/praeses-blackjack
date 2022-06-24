//! Logic for Blackjack players: how they decide what to do and
//! what happens when they make their action are here. Using this,
//! you can create players that behave differently but all act
//! within the allowed moves in Blackjack.

use crate::blackjack;
use crate::cards;
use std::io;

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
        let input = &input.to_lowercase()[..];
        match input {
            "hit" | "h" => Ok(Self::Hit),
            "stand" | "s" => Ok(Self::Stand),
            _ => Err("Invalid action input"),
        }
    }
}

/// A trait representing behavior every player in a game of blackjack should be able to handle.
pub trait BlackjackPlayer {
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

    /// Get what action a player should take.
    fn get_action(&self) -> Action;

    /// Carry out a player's actions in the game.
    /// Returns true or false if they can take another turn or not.
    fn handle_player_action(&mut self, action: Action, deck: &mut cards::Deck) -> bool {
        match action {
            Action::Hit => {
                let deal = deck.pop().unwrap();
                println!("Hit! NEW CARD: {}", deal);
                self.recieve_card(deal);
                false
            }
            Action::Stand => true,
        }
    }

    /// A player's turn logic is in here!
    fn take_turn(&mut self, deck: &mut cards::Deck) -> bool {
        let action = self.get_action();
        self.handle_player_action(action, deck)
    }

    //fn new() -> Self {
    //    Self {
    //        hand: Vec::new()
    //    }
    //}
}

/// A player controlled by a human and their input into the commandline. Their output is sent to stdout.
pub struct HumanPlayer {
    hand: cards::Hand,
}

impl BlackjackPlayer for HumanPlayer {
    fn get_action(&self) -> Action {
        println!("{}", Action::ACTION_PROMPT);

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            match Action::parse_from_string(&input) {
                Ok(action) => return action,
                Err(e) => println!("{}, try again.", e),
            }
        }
    }

    fn get_hand(&self) -> &Vec<cards::Card> {
        &self.hand
    }

    fn show_hand(&self) {
        print!("Cards: {}", &self.hand[0]);
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!(
            "     (value: {})",
            blackjack::get_hand_value(&self.hand[..])
        );
    }

    fn recieve_card(&mut self, card: cards::Card) {
        self.hand.push(card);
    }
}

impl HumanPlayer {
    /// Creates a new human player.
    pub fn new() -> HumanPlayer {
        HumanPlayer { hand: Vec::new() }
    }
}

/// A trait representing the dealer in a game of blackjack.
/// They act similarly to players, but with a bit more behaviors needed.
pub trait BlackjackDealer: BlackjackPlayer {
    /// Shows the tur ehand of the dealer (because usually their complete hand will be hidden from players).
    fn show_true_hand(&self);

    /// Creates a new BlackjackDealer
    fn new() -> Self;
}

/// A standard dealer whose output is sent to stdout.
pub struct Dealer {
    hand: cards::Hand,
}

// Dealer probably doesn't need to implement this actually...
impl BlackjackPlayer for Dealer {
    fn get_action(&self) -> Action {
        if blackjack::get_hand_value(&self.hand[..]) >= 17 {
            Action::Stand
        } else {
            Action::Hit
        }
    }

    fn get_hand(&self) -> &Vec<cards::Card> {
        &self.hand
    }

    fn show_hand(&self) {
        print!("Dealer's Cards: **");
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!("");
    }

    fn recieve_card(&mut self, card: cards::Card) {
        self.hand.push(card);
    }
}

impl BlackjackDealer for Dealer {
    fn show_true_hand(&self) {
        print!("Dealer's Cards: {}", &self.hand[0]);
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!(
            "     (value: {})",
            blackjack::get_hand_value(&self.hand[..])
        );
    }

    fn new() -> Dealer {
        Dealer { hand: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_card_to_hand<T: BlackjackPlayer>(mut player: T) {
        assert_eq!(0, player.get_hand().len());
        player.recieve_card(cards::Card {rank: cards::Rank::Ace, suit: cards::Suit::Spade});

        assert_eq!(1, player.get_hand().len());
    }

    #[test]
    fn player_adds_card_to_hand() {
        add_card_to_hand(HumanPlayer::new());
    }

    #[test]
    fn dealer_adds_card_to_hand() {
        add_card_to_hand(Dealer::new());
    }

    #[test]
    fn dealer_acts_properly() {
        let mut dealer = Dealer::new();

        dealer.recieve_card(cards::Card {rank: cards::Rank::Seven, suit: cards::Suit::Spade});
        dealer.recieve_card(cards::Card {rank: cards::Rank::Ten, suit: cards::Suit::Spade});
        assert_eq!(dealer.get_action(), Action::Stand);
        let mut dealer = Dealer::new();

        dealer.recieve_card(cards::Card {rank: cards::Rank::King, suit: cards::Suit::Spade});
        dealer.recieve_card(cards::Card {rank: cards::Rank::Queen, suit: cards::Suit::Spade});
        assert_eq!(dealer.get_action(), Action::Stand);
        let mut dealer = Dealer::new();

        dealer.recieve_card(cards::Card {rank: cards::Rank::Seven, suit: cards::Suit::Spade});
        dealer.recieve_card(cards::Card {rank: cards::Rank::Ace, suit: cards::Suit::Spade});
        assert_eq!(dealer.get_action(), Action::Stand);
        let mut dealer = Dealer::new();

        dealer.recieve_card(cards::Card {rank: cards::Rank::Four, suit: cards::Suit::Spade});
        dealer.recieve_card(cards::Card {rank: cards::Rank::Eight, suit: cards::Suit::Spade});
        assert_eq!(dealer.get_action(), Action::Hit);
        let mut dealer = Dealer::new();

        dealer.recieve_card(cards::Card {rank: cards::Rank::Nine, suit: cards::Suit::Spade});
        dealer.recieve_card(cards::Card {rank: cards::Rank::Seven, suit: cards::Suit::Spade});
        assert_eq!(dealer.get_action(), Action::Hit);
    }
}
