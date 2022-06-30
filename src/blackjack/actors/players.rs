//! Player-specific logic. Players need to worry about betting and can take actions
//! that dealers can't, so this is for modeling players who are sitting at the casino table.
//! Using this, you can create players that behave differently but all
//! act within the allowed moves in Blackjack.

pub mod auto_player;
pub mod human_player;

pub use auto_player::AutoPlayer;
pub use human_player::HumanPlayer;

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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::blackjack::actors;
    use crate::blackjack::actors::tests as actor_tests;

    /// Helper function for checking player actions given their cards and what they can see from the dealer.
    pub fn check_action_from_cards<T: Player>(
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
}
