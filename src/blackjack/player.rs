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
    /// Creates a new BlackjackPlayer
    fn new() -> Self
    where
        Self: Sized;

    fn get_name(&self) -> &str;

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
    fn get_action(&self, dealer_upcard: &cards::Card) -> Action;

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
    fn take_turn(&mut self, deck: &mut cards::Deck, dealer_upcard: &cards::Card) -> bool {
        let action = self.get_action(dealer_upcard);
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
    name: String,
    hand: cards::Hand,
}

impl BlackjackPlayer for HumanPlayer {
    fn new() -> HumanPlayer {
        println!("Input your name (or leave blank to be Player)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        if input.is_empty() {
            input = "Player".to_owned();
        }

        HumanPlayer {
            name: input,
            hand: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_action(&self, _dealer_upcard: &cards::Card) -> Action {
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
        print!("{}'s Cards: {}", self.get_name(), &self.hand[0]);
        for card in &self.get_hand_slice()[1..] {
            print!(", {}", card);
        }
        println!(
            "     (value: {})",
            blackjack::get_hand_value(&self.get_hand_slice())
        );
    }

    fn recieve_card(&mut self, card: cards::Card) {
        self.hand.push(card);
    }
}

/// A trait representing the dealer in a game of blackjack.
/// They act similarly to players, but with a bit more behaviors needed.
pub trait BlackjackDealer: BlackjackPlayer {
    /// Shows the true hand of the dealer (because usually their complete hand will be hidden from players).
    fn show_true_hand(&self);
}

/// A standard dealer whose output is sent to stdout.
pub struct Dealer {
    hand: cards::Hand,
}

// Dealer probably doesn't need to implement this actually...
impl BlackjackPlayer for Dealer {
    fn new() -> Dealer {
        Dealer { hand: Vec::new() }
    }

    fn get_name(&self) -> &str {
        "Dealer"
    }

    fn get_action(&self, _dealer_upcard: &cards::Card) -> Action {
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
}

/// A simple bot acting as a player that will always do the most optimal move
/// Given their hand and without counting cards.
pub struct AutoPlayer {
    hand: cards::Hand,
}

impl BlackjackPlayer for AutoPlayer {
    fn new() -> AutoPlayer {
        AutoPlayer { hand: Vec::new() }
    }

    fn get_name(&self) -> &str {
        "Bot"
    }

    fn get_action(&self, dealer_upcard: &cards::Card) -> Action {
        // If the player has a soft hand (hand with ace), hit until at least 18.
        if self
            .get_hand_slice()
            .iter()
            .any(|&x| x.rank == cards::Rank::Ace)
        {
            if blackjack::get_hand_value(self.get_hand_slice()) >= 18 {
                return Action::Stand;
            } else {
                return Action::Hit;
            }
        }

        let stop_at: u32;
        match dealer_upcard.rank {
            cards::Rank::Ace
            | cards::Rank::Seven
            | cards::Rank::Eight
            | cards::Rank::Nine
            | cards::Rank::Ten
            | cards::Rank::Jack
            | cards::Rank::Queen
            | cards::Rank::King => stop_at = 17,
            cards::Rank::Four | cards::Rank::Five | cards::Rank::Six => stop_at = 12,
            cards::Rank::Two | cards::Rank::Three => stop_at = 13,
        }

        if blackjack::get_hand_value(self.get_hand_slice()) >= stop_at {
            Action::Stand
        } else {
            Action::Hit
        }
    }

    fn get_hand(&self) -> &Vec<cards::Card> {
        &self.hand
    }

    fn show_hand(&self) {
        print!("{}'s Cards: {}", self.get_name(), &self.hand[0]);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn add_card_to_hand<T: BlackjackPlayer>(mut player: T) {
        assert_eq!(0, player.get_hand().len());
        player.recieve_card(cards::Card {
            rank: cards::Rank::Ace,
            suit: cards::Suit::Spade,
        });

        assert_eq!(1, player.get_hand().len());
    }

    fn create_card_from_value(value: u32) -> cards::Card {
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

    // Broken because it waits for a name input, looking into solutions
    //#[test]
    //fn player_adds_card_to_hand() {
    //    add_card_to_hand(HumanPlayer::new());
    //}

    #[test]
    fn dealer_adds_card_to_hand() {
        add_card_to_hand(Dealer::new());
    }

    #[test]
    fn dealer_acts_properly() {
        let upcard = create_card_from_value(1);

        let mut dealer = Dealer::new();
        dealer.recieve_card(create_card_from_value(7));
        dealer.recieve_card(create_card_from_value(10));
        assert_eq!(dealer.get_action(&upcard), Action::Stand);

        let mut dealer = Dealer::new();
        dealer.recieve_card(create_card_from_value(10));
        dealer.recieve_card(create_card_from_value(10));
        assert_eq!(dealer.get_action(&upcard), Action::Stand);

        let mut dealer = Dealer::new();
        dealer.recieve_card(create_card_from_value(7));
        dealer.recieve_card(create_card_from_value(1));
        assert_eq!(dealer.get_action(&upcard), Action::Stand);

        let mut dealer = Dealer::new();
        dealer.recieve_card(create_card_from_value(4));
        dealer.recieve_card(create_card_from_value(8));
        assert_eq!(dealer.get_action(&upcard), Action::Hit);

        let mut dealer = Dealer::new();
        dealer.recieve_card(create_card_from_value(9));
        dealer.recieve_card(create_card_from_value(7));
        assert_eq!(dealer.get_action(&upcard), Action::Hit);
    }

    #[test]
    fn bot_acts_properly() {
        // If you have an ace, stand at value of 18 or more.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(5);
        bot.recieve_card(create_card_from_value(1));
        bot.recieve_card(create_card_from_value(7));
        assert_eq!(bot.get_action(&upcard), Action::Stand);

        // If you have an ace, hit at a value of 17 or less.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(5);
        bot.recieve_card(create_card_from_value(1));
        bot.recieve_card(create_card_from_value(6));
        assert_eq!(bot.get_action(&upcard), Action::Hit);

        // If the dealer's card is good, stand at 17 or more.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(10);
        bot.recieve_card(create_card_from_value(10));
        bot.recieve_card(create_card_from_value(7));
        assert_eq!(bot.get_action(&upcard), Action::Stand);

        // If the dealer's card is good, hit at 16 or less.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(10);
        bot.recieve_card(create_card_from_value(10));
        bot.recieve_card(create_card_from_value(6));
        assert_eq!(bot.get_action(&upcard), Action::Hit);

        // If the dealer's card is bad, stand at 12 or more.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(4);
        bot.recieve_card(create_card_from_value(10));
        bot.recieve_card(create_card_from_value(2));
        assert_eq!(bot.get_action(&upcard), Action::Stand);

        // If the dealer's card is bad, hit at 11 or less.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(4);
        bot.recieve_card(create_card_from_value(8));
        bot.recieve_card(create_card_from_value(3));
        assert_eq!(bot.get_action(&upcard), Action::Hit);

        // If the dealer's card is fair, stand at 13 or more.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(2);
        bot.recieve_card(create_card_from_value(10));
        bot.recieve_card(create_card_from_value(3));
        assert_eq!(bot.get_action(&upcard), Action::Stand);

        // If the dealer's card is fair, hit at 12 or less.
        let mut bot = AutoPlayer::new();
        let upcard = create_card_from_value(2);
        bot.recieve_card(create_card_from_value(10));
        bot.recieve_card(create_card_from_value(2));
        assert_eq!(bot.get_action(&upcard), Action::Hit);
    }
}
