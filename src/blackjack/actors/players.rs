//! Player-specific logic. Players need to worry about betting and can take actions
//! that dealers can't, so this is for modeling players who are sitting at the casino table.
//! Using this, you can create players that behave differently but all
//! act within the allowed moves in Blackjack.

use std::io;

use crate::blackjack::actors::Actor;
use crate::blackjack::{self, actors};
use crate::cards;

/// A trait representing behavior every player in a game of blackjack should be able to handle.
pub trait Player: actors::Actor {
    /// Creates a new object that implements Player.
    fn new(buyin: u32) -> Self
    where
        Self: Sized;

    /// Returns a string slice representing this player's name.
    fn get_name(&self) -> &str;

    /// Gets the total money a player has currently.
    fn get_money(&mut self) -> &mut Option<u32>;

    /// Gets how much money a player is betting on the current round.
    fn get_bet(&mut self) -> &mut Option<u32>;

    /// Solicits how much a player wants to bet and puts that money aside for betting.
    fn set_bet(&mut self);

    /// Gives the player more money if they are out of it to keep the game going.  
    fn buy_in_if_broke(&mut self, buy_in_amount: u32) {
        if *self.get_money() == Some(0) {
            println!(
                "You went broke, {}! Don't worry, I'll spot you some cash.",
                self.get_name()
            );
            *self.get_money() = Some(buy_in_amount);
        }
    }

    /// Get what action a player should take.
    fn decide_action(&self, dealer_upcard: &cards::Card) -> actors::Action;

    /// Carry out a player's actions in the game.
    /// Returns true or false if they can take another turn or not.
    fn handle_player_action(&mut self, action: actors::Action, deck: &mut cards::Deck) -> bool {
        match action {
            actors::Action::Hit => {
                let deal = deck.pop().unwrap();
                println!("Hit! NEW CARD: {}", deal);
                self.recieve_card(deal);
                false
            }
            actors::Action::Stand => true,
        }
    }

    /// Decide what action to take and handle that action. Returns true if they can take another turn.
    fn take_turn(&mut self, deck: &mut cards::Deck, dealer_upcard: &cards::Card) -> bool {
        let action = self.decide_action(dealer_upcard);
        self.handle_player_action(action, deck)
    }

    /// Handles the result for a player at the end of a round (showing it to the user, updating bet/money).
    fn handle_round_result(&mut self, result: blackjack::PlayerRoundResult, payout_ratio: f64) {
        print!("{}: {} ", self.get_name(), result);
        if self.get_bet().is_none() {
            println!("");
            return;
        }

        let bet = self.get_bet().unwrap();
        match result {
            blackjack::PlayerRoundResult::Natural => {
                let winnings = bet + (payout_ratio * bet as f64).floor() as u32;
                *self.get_money() = Some(self.get_money().unwrap() + winnings);
                println!(
                    "You won ${}. (Total cash: ${})",
                    winnings,
                    self.get_money().unwrap()
                );
            }
            blackjack::PlayerRoundResult::Win => {
                let winnings: u32 = bet + bet;
                *self.get_money() = Some(self.get_money().unwrap() + winnings);
                println!(
                    "You won ${}. (Total cash: ${})",
                    winnings,
                    self.get_money().unwrap()
                );
            }
            blackjack::PlayerRoundResult::Standoff => {
                *self.get_money() = Some(self.get_money().unwrap() + bet);
                println!(
                    "You kept your original ${} bet (Total cash: ${})",
                    bet,
                    self.get_money().unwrap()
                );
            }
            blackjack::PlayerRoundResult::Lose => {
                println!(
                    "You lost your ${} bet. (Total cash: ${})",
                    bet,
                    self.get_money().unwrap()
                );
            }
        }
        *self.get_bet() = None;
    }
}

/// A player controlled by a human and their input into the terminal. Their output is sent to stdout.
pub struct HumanPlayer {
    name: String,
    hand: cards::Hand,
    money: Option<u32>,
    bet: Option<u32>,
}

impl actors::Actor for HumanPlayer {
    fn get_hand(&mut self) -> &mut Vec<cards::Card> {
        &mut self.hand
    }

    fn get_hand_slice(&self) -> &[cards::Card] {
        self.hand.as_slice()
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
}

impl Player for HumanPlayer {
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

    fn get_money(&mut self) -> &mut Option<u32> {
        &mut self.money
    }

    fn get_bet(&mut self) -> &mut Option<u32> {
        &mut self.bet
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

    fn decide_action(&self, _dealer_upcard: &cards::Card) -> actors::Action {
        println!("{}", actors::Action::ACTION_PROMPT);

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match actors::Action::parse_from_string(&input) {
                Ok(action) => return action,
                Err(e) => println!("{}, try again.", e),
            }
        }
    }
}

impl HumanPlayer {
    /// Used in testing to not need person's input to create a HumanPlayer.
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

/// A simple bot acting as a player that will always do the most optimal move
/// given their hand without counting cards.
pub struct AutoPlayer {
    hand: cards::Hand,
    money: Option<u32>,
    bet: Option<u32>,
}

impl actors::Actor for AutoPlayer {
    fn get_hand(&mut self) -> &mut Vec<cards::Card> {
        &mut self.hand
    }

    fn get_hand_slice(&self) -> &[cards::Card] {
        self.hand.as_slice()
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
}

impl Player for AutoPlayer {
    fn new(buy_in: u32) -> AutoPlayer {
        let money = if buy_in > 0 { Some(buy_in) } else { None };

        AutoPlayer {
            hand: Vec::new(),
            money,
            bet: None,
        }
    }

    fn get_name(&self) -> &str {
        "Bot"
    }

    fn get_money(&mut self) -> &mut Option<u32> {
        &mut self.money
    }

    fn get_bet(&mut self) -> &mut Option<u32> {
        &mut self.bet
    }

    fn set_bet(&mut self) {
        // Maybe put in bot betting logic.
        //println!("Getting bet for {}", self.get_name());
    }

    fn decide_action(&self, dealer_upcard: &cards::Card) -> actors::Action {
        // If the player has a soft hand, hit until at least 18.
        if blackjack::is_soft_hand(
            blackjack::get_raw_hand_value(self.get_hand_slice()),
            self.get_hand_slice(),
        ) {
            if blackjack::get_hand_value(self.get_hand_slice()) >= 18 {
                return actors::Action::Stand;
            } else {
                return actors::Action::Hit;
            }
        }

        let stop_at = match dealer_upcard.rank {
            // Good hands
            cards::Rank::Ace
            | cards::Rank::Seven
            | cards::Rank::Eight
            | cards::Rank::Nine
            | cards::Rank::Ten
            | cards::Rank::Jack
            | cards::Rank::Queen
            | cards::Rank::King => 17,
            // Poor hands
            cards::Rank::Four | cards::Rank::Five | cards::Rank::Six => 12,
            // Fair hands
            cards::Rank::Two | cards::Rank::Three => 13,
        };

        if blackjack::get_hand_value(self.get_hand_slice()) >= stop_at {
            actors::Action::Stand
        } else {
            actors::Action::Hit
        }
    }
}

impl AutoPlayer {
    /// Used in testing to not need person's input to create a HumanPlayer.
    #[allow(dead_code)]
    fn new_default() -> AutoPlayer {
        AutoPlayer {
            hand: Vec::new(),
            money: None,
            bet: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests as actor_tests;
    use super::*;
    use crate::blackjack::actors;

    /// Helper function for checking player actions given their cards and what they can see from the dealer.
    fn check_action_from_cards<T: Player>(
        card_values: (u32, u32),
        upcard: u32,
        action: actors::Action,
    ) {
        let upcard = actor_tests::create_card_from_value(upcard);
        let mut player = T::new(0);
        player.recieve_card(actor_tests::create_card_from_value(card_values.0));
        player.recieve_card(actor_tests::create_card_from_value(card_values.1));
        assert_eq!(player.decide_action(&upcard), action);
    }

    /// Check that
    #[test]
    fn human_player_adds_card_to_hand() {
        actor_tests::add_card_to_hand(HumanPlayer::new_default());
    }

    #[test]
    fn bot_player_adds_card_to_hand() {
        actor_tests::add_card_to_hand(AutoPlayer::new_default());
    }

    #[test]
    fn bot_acts_properly() {
        // If you have an ace, stand at value of 18 or more.
        check_action_from_cards::<AutoPlayer>((1, 7), 1, actors::Action::Stand);

        // If you have an ace, hit at a value of 17 or less.
        check_action_from_cards::<AutoPlayer>((1, 6), 1, actors::Action::Hit);

        // If the dealer's card is good, stand at 17 or more.
        check_action_from_cards::<AutoPlayer>((10, 7), 10, actors::Action::Stand);

        // If the dealer's card is good, hit at 16 or less.
        check_action_from_cards::<AutoPlayer>((10, 6), 10, actors::Action::Hit);

        // If the dealer's card is bad, stand at 12 or more.
        check_action_from_cards::<AutoPlayer>((10, 2), 4, actors::Action::Stand);

        // If the dealer's card is bad, hit at 11 or less.
        check_action_from_cards::<AutoPlayer>((8, 3), 4, actors::Action::Hit);

        // If the dealer's card is fair, stand at 13 or more.
        check_action_from_cards::<AutoPlayer>((10, 3), 2, actors::Action::Stand);

        // If the dealer's card is fair, hit at 12 or less.
        check_action_from_cards::<AutoPlayer>((10, 2), 2, actors::Action::Hit);
    }
}
