use std::io;

use crate::blackjack::actors::players;
use crate::blackjack::actors::players::Player;
use crate::blackjack::{self, actors};
use crate::cards;

/// A player controlled by a human and their input into the terminal. Their output is sent to stdout.
pub struct HumanPlayer {
    name: String,
    hand: cards::Hand,
    money: Option<u32>,
    bet: Option<u32>,
}

impl actors::Actor for HumanPlayer {
    fn hand_mut(&mut self) -> &mut Vec<cards::Card> {
        &mut self.hand
    }

    fn hand(&self) -> &[cards::Card] {
        self.hand.as_slice()
    }

    fn show_hand(&self) {
        print!("{}'s Cards: {}", self.name(), &self.hand[0]);
        for card in &self.hand()[1..] {
            print!(", {}", card);
        }
        println!("     (value: {})", blackjack::hand_value(&self.hand()));
    }
}

impl players::Player for HumanPlayer {
    fn new(buy_in: u32) -> HumanPlayer {
        println!("Input your name (or leave blank to be Player)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        if input.is_empty() {
            input = "Player".to_owned();
        }

        let money: Option<u32> = if buy_in > 0 { Some(buy_in) } else { None };

        HumanPlayer {
            name: input,
            hand: Vec::new(),
            money,
            bet: None,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn money_mut(&mut self) -> &mut Option<u32> {
        &mut self.money
    }

    fn bet_mut(&mut self) -> &mut Option<u32> {
        &mut self.bet
    }

    fn place_bet(&mut self) {
        let funds = self.money_mut();
        if funds.is_none() {
            return;
        }
        let funds = funds.unwrap();

        println!(
            "What would you like to bet this round, {}? (Funds: ${}) ",
            self.name(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blackjack::actors::tests as actor_tests;

    /// Check that
    #[test]
    fn human_player_adds_card_to_hand() {
        actor_tests::adds_card_to_hand(HumanPlayer::new_default());
    }
}
