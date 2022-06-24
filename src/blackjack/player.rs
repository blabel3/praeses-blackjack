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
        let input = &input.trim().to_lowercase()[..];
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
    fn new(buyin: u32) -> Self
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

    fn get_money(&self) -> &Option<u32> {
        &None
    }

    fn set_bet(&mut self) {
        ()
    }

    fn get_bet(&self) -> &Option<u32> {
        &None
    }

    fn buyin_if_broke(&mut self, buyin_amount: u32);

    fn handle_round_result(&mut self, result: blackjack::PlayerRoundResult, payout_ratio: f64);

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
    money: Option<u32>,
    bet: Option<u32>,
}

impl BlackjackPlayer for HumanPlayer {
    fn new(buyin: u32) -> HumanPlayer {
        println!("Input your name (or leave blank to be Player)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        if input.is_empty() {
            input = "Player".to_owned();
        }

        let money: Option<u32> = if buyin > 0 { Some(buyin) } else { None };

        HumanPlayer {
            name: input,
            hand: Vec::new(),
            money,
            bet: None,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_money(&self) -> &Option<u32> {
        &self.money
    }

    fn buyin_if_broke(&mut self, buyin_amount: u32) {
        if self.money == Some(0) {
            println!(
                "You went broke, {}! Don't worry, I'll spot you some cash.",
                self.get_name()
            );
            self.money = Some(buyin_amount);
        }
    }

    fn set_bet(&mut self) {
        let funds = self.get_money();
        if funds.is_none() {
            return;
        }
        let funds = funds.unwrap();

        println!(
            "What would you like to bet this round, {}? (Funds: ${}) ",
            self.get_name(),
            funds
        );

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            let input = if input.starts_with('$') {
                &input[1..]
            } else {
                input
            };

            if input == "" || input == "0" {
                println!("Not betting this round.");
                return;
            }

            match input.parse::<u32>() {
                Ok(number) => {
                    if number > funds {
                        println!("You don't have that kind of cash!");
                    } else {
                        println!("Betting ${}.", number);
                        self.bet = Some(number);
                        self.money = Some(funds - number);
                        return;
                    }
                }
                Err(_e) => println!("Didn't catch that, try again."),
            }
        }
    }

    fn get_bet(&self) -> &Option<u32> {
        &self.bet
    }

    fn handle_round_result(&mut self, result: blackjack::PlayerRoundResult, payout_ratio: f64) {
        self.hand.clear();
        print!("{}: {} ", self.get_name(), result);
        if self.get_bet().is_none() {
            println!("");
        } else {
            let bet = self.get_bet().unwrap();
            match result {
                blackjack::PlayerRoundResult::Natural => {
                    let winnings = bet + (payout_ratio * bet as f64).floor() as u32;
                    self.money = Some(self.money.unwrap() + winnings);
                    self.bet = None;
                    println!(
                        "You won: ${} (Total cash: ${})",
                        winnings,
                        self.get_money().unwrap()
                    );
                }
                blackjack::PlayerRoundResult::Win => {
                    let winnings: u32 = bet + bet;
                    self.money = Some(self.money.unwrap() + winnings);
                    self.bet = None;
                    println!(
                        "You won: ${} (Total cash: ${})",
                        winnings,
                        self.get_money().unwrap()
                    );
                }
                blackjack::PlayerRoundResult::Standoff => {
                    self.money = Some(self.money.unwrap() + bet);
                    self.bet = None;
                    println!(
                        "You kept your original bet ${} (Total cash: ${})",
                        bet,
                        self.get_money().unwrap()
                    );
                    return;
                }
                blackjack::PlayerRoundResult::Lose => {
                    self.bet = None;
                    println!(
                        "You lost your bet ${} (Total cash: ${})",
                        bet,
                        self.get_money().unwrap()
                    );
                    return;
                }
            }
        }
    }

    fn get_action(&self, _dealer_upcard: &cards::Card) -> Action {
        println!("{}", Action::ACTION_PROMPT);

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

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

impl HumanPlayer {
    /// Used in testing to not need person's input.
    #[allow(dead_code)]
    fn new_default() -> HumanPlayer {
        HumanPlayer {
            name: "Player".to_string(),
            hand: Vec::new(),
            money: None,
            bet: None,
        }
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
    fn new(_buyin: u32) -> Dealer {
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

    fn handle_round_result(&mut self, _result: blackjack::PlayerRoundResult, _payout_ratio: f64) {
        ()
    }

    fn buyin_if_broke(&mut self, _buyin_amount: u32) {
        ()
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
    money: Option<u32>,
    bet: Option<u32>,
}

impl BlackjackPlayer for AutoPlayer {
    fn new(buyin: u32) -> AutoPlayer {
        let money = if buyin > 0 { Some(buyin) } else { None };

        AutoPlayer {
            hand: Vec::new(),
            money,
            bet: None,
        }
    }

    fn get_name(&self) -> &str {
        "Bot"
    }

    fn get_money(&self) -> &Option<u32> {
        &self.money
    }

    fn set_bet(&mut self) {
        // Maybe put in bot betting logic.
        //println!("Getting bet for {}", self.get_name());
    }

    fn get_bet(&self) -> &Option<u32> {
        &self.bet
    }

    fn buyin_if_broke(&mut self, buyin_amount: u32) {
        if self.money == Some(0) {
            self.money = Some(buyin_amount);
        }
    }

    fn get_action(&self, dealer_upcard: &cards::Card) -> Action {
        // If the player has a soft hand, hit until at least 18.

        if blackjack::is_soft_hand(
            blackjack::get_raw_hand_value(self.get_hand_slice()),
            self.get_hand_slice(),
        ) {
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

    fn handle_round_result(&mut self, result: blackjack::PlayerRoundResult, payout_ratio: f64) {
        self.hand.clear();
        print!("{}: {} ", self.get_name(), result);
        if self.get_bet().is_none() {
            println!("");
        } else {
            let bet = self.get_bet().unwrap();
            match result {
                blackjack::PlayerRoundResult::Natural => {
                    let winnings = bet + (payout_ratio * bet as f64).floor() as u32;
                    self.money = Some(self.money.unwrap() + winnings);
                    self.bet = None;
                    println!(
                        "You won ${} (Total cash: ${})",
                        winnings,
                        self.get_money().unwrap()
                    );
                }
                blackjack::PlayerRoundResult::Win => {
                    let winnings: u32 = bet + bet;
                    self.money = Some(self.money.unwrap() + winnings);
                    self.bet = None;
                    println!(
                        "You won ${} (Total cash: ${})",
                        winnings,
                        self.get_money().unwrap()
                    );
                }
                blackjack::PlayerRoundResult::Standoff => {
                    self.money = Some(self.money.unwrap() + bet);
                    self.bet = None;
                    println!(
                        "You kept your original ${} bet (Total cash: ${})",
                        bet,
                        self.get_money().unwrap()
                    );
                    return;
                }
                blackjack::PlayerRoundResult::Lose => {
                    self.bet = None;
                    println!(
                        "You lost your ${} bet. (Total cash: ${})",
                        bet,
                        self.get_money().unwrap()
                    );
                    return;
                }
            }
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

    fn check_action_from_cards<T: BlackjackPlayer>(
        card_values: (u32, u32),
        upcard: Option<u32>,
        action: Action,
    ) {
        let upcard = create_card_from_value(upcard.unwrap_or(1));
        let mut player = T::new(0);
        player.recieve_card(create_card_from_value(card_values.0));
        player.recieve_card(create_card_from_value(card_values.1));
        assert_eq!(player.get_action(&upcard), action);
    }

    #[test]
    fn player_adds_card_to_hand() {
        add_card_to_hand(HumanPlayer::new_default());
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

    #[test]
    fn dealer_adds_card_to_hand() {
        add_card_to_hand(Dealer::new(0));
    }

    #[test]
    fn dealer_acts_properly() {
        check_action_from_cards::<Dealer>((7, 10), None, Action::Stand);
        check_action_from_cards::<Dealer>((10, 10), None, Action::Stand);
        check_action_from_cards::<Dealer>((7, 1), None, Action::Stand);
        check_action_from_cards::<Dealer>((4, 8), None, Action::Hit);
        check_action_from_cards::<Dealer>((9, 7), None, Action::Hit);
    }

    #[test]
    fn bot_acts_properly() {
        // If you have an ace, stand at value of 18 or more.
        check_action_from_cards::<AutoPlayer>((1, 7), None, Action::Stand);

        // If you have an ace, hit at a value of 17 or less.
        check_action_from_cards::<AutoPlayer>((1, 6), None, Action::Hit);

        // If the dealer's card is good, stand at 17 or more.
        check_action_from_cards::<AutoPlayer>((10, 7), Some(10), Action::Stand);

        // If the dealer's card is good, hit at 16 or less.
        check_action_from_cards::<AutoPlayer>((10, 6), Some(10), Action::Hit);

        // If the dealer's card is bad, stand at 12 or more.
        check_action_from_cards::<AutoPlayer>((10, 2), Some(4), Action::Stand);

        // If the dealer's card is bad, hit at 11 or less.
        check_action_from_cards::<AutoPlayer>((8, 3), Some(4), Action::Hit);

        // If the dealer's card is fair, stand at 13 or more.
        check_action_from_cards::<AutoPlayer>((10, 3), Some(2), Action::Stand);

        // If the dealer's card is fair, hit at 12 or less.
        check_action_from_cards::<AutoPlayer>((10, 2), Some(2), Action::Hit);
    }
}
