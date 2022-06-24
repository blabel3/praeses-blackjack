use crate::blackjack::{self, actors};
use crate::cards;

/// A trait representing the dealer in a game of blackjack.
/// They act similarly to players, but with a bit more behaviors needed.
pub trait Dealer: actors::Actor {
    /// Creates a new Dealer
    fn new() -> Self
    where
        Self: Sized;

    /// Shows the true hand of the dealer (because usually their complete hand will be hidden from players).
    fn show_true_hand(&self);

    /// Get what action a dealer should take. Should be the same for all dealers 
    /// so a default implementation is provided.
    fn decide_action(&self) -> actors::Action {
        if blackjack::get_hand_value(self.get_hand_slice()) >= 17 {
            actors::Action::Stand
        } else {
            actors::Action::Hit
        }
    }

    /// Carry out a dealer's actions in the game. Dopesn't depend on anything but their cards. 
    /// Returns true or false if they can take another turn or not.
    fn handle_dealer_action(&mut self, action: actors::Action, deck: &mut cards::Deck) -> bool {
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

    /// A dealer's turn logic is in here!
    fn take_turn(&mut self, deck: &mut cards::Deck) -> bool {
        let action = self.decide_action();
        self.handle_dealer_action(action, deck)
    }
}

/// A standard dealer whose output is sent to stdout.
pub struct StandardDealer {
    hand: cards::Hand,
}

impl actors::Actor for StandardDealer {
    fn get_hand(&mut self) -> &mut Vec<cards::Card> {
        &mut self.hand
    }

    fn get_hand_slice(&self) -> &[cards::Card] {
        self.hand.as_slice()
    }

    fn show_hand(&self) {
        print!("Dealer's Cards: **");
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!("");
    }
}

impl Dealer for StandardDealer {
    fn new() -> StandardDealer {
        StandardDealer { hand: Vec::new() }
    }

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

#[cfg(test)]
mod tests {
    use super::super::tests as actor_tests;
    use super::*;
    use crate::blackjack::actors;

    /// Helper funciton for checking that a dealer's action is proper.
    fn check_action_from_cards<T: Dealer>(card_values: (u32, u32), action: actors::Action) {
        let mut dealer = T::new();
        dealer.recieve_card(actor_tests::create_card_from_value(card_values.0));
        dealer.recieve_card(actor_tests::create_card_from_value(card_values.1));
        assert_eq!(dealer.decide_action(), action);
    }

    /// Making sure the dealer can add a card to their hand. 
    #[test]
    fn standard_dealer_adds_card_to_hand() {
        actor_tests::add_card_to_hand(StandardDealer::new());
    }

    /// Check that the dealer's actions follow blackjack rules. 
    #[test]
    fn standard_dealer_acts_properly() {
        // Dealer has to stand at 17. 
        check_action_from_cards::<StandardDealer>((7, 10), actors::Action::Stand);

        // Dealer should also stand at a soft 18. 
        check_action_from_cards::<StandardDealer>((7, 1), actors::Action::Stand);

        // Dealer should hit at 12. 
        check_action_from_cards::<StandardDealer>((4, 8), actors::Action::Hit);

        // Dealer should also hit at a soft 13. 
        check_action_from_cards::<StandardDealer>((1, 2), actors::Action::Hit);
    }
}
