//! Logic for Blackjack players: how they decide what to do,
//! what happens when they make their action, and all of that
//! are here so that we can make players that behave differently
//! that all act within the allowed moves in Blackjack.

use crate::blackjack;
use crate::cards;
use std::io;

/// Supported player actions.
#[derive(Debug)]
pub enum Action {
    /// Hit: adds a card from the deck to your hand
    Hit,
    /// Stand: keep the cards in your hand and pass to the next player
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

pub trait BlackjackPlayer {
    fn get_hand(&self) -> &Vec<cards::Card>;

    fn show_hand(&self) -> ();

    // Can probably turn new and this into a macro maybe?
    fn recieve_card(&mut self, card: cards::Card) -> ();

    fn get_action(&self) -> Action;

    fn handle_player_action(&mut self, action: Action, deck: &mut Vec<cards::Card>) -> bool {
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

    fn take_turn(&mut self, deck: &mut Vec<cards::Card>) -> bool {
        let action = self.get_action();
        self.handle_player_action(action, deck)
    }

    //fn new() -> Self {
    //    Self {
    //        hand: Vec::new()
    //    }
    //}
}

pub struct HumanPlayer {
    pub hand: Vec<cards::Card>,
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
    pub fn new() -> HumanPlayer {
        HumanPlayer { hand: Vec::new() }
    }
}

pub trait BlackjackDealer: BlackjackPlayer {
    fn show_true_hand(&self);

    fn new() -> Self;
}

pub struct Dealer {
    pub hand: Vec<cards::Card>,
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
