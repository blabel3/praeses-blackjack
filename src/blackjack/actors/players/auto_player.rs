use crate::blackjack::actors::players;
use crate::blackjack::actors::players::Player;
use crate::blackjack::actors::Actor;
use crate::blackjack::{self, actors};
use crate::cards;

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

impl players::Player for AutoPlayer {
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
    use super::*;
    use crate::blackjack::actors;
    use crate::blackjack::actors::players::tests as players_tests;
    use crate::blackjack::actors::tests as actor_tests;

    #[test]
    fn bot_player_adds_card_to_hand() {
        actor_tests::adds_card_to_hand(AutoPlayer::new_default());
    }

    #[test]
    fn bot_acts_properly() {
        // If you have an ace, stand at value of 18 or more.
        players_tests::check_action_from_cards::<AutoPlayer>((1, 7), 1, actors::Action::Stand);

        // If you have an ace, hit at a value of 17 or less.
        players_tests::check_action_from_cards::<AutoPlayer>((1, 6), 1, actors::Action::Hit);

        // If the dealer's card is good, stand at 17 or more.
        players_tests::check_action_from_cards::<AutoPlayer>((10, 7), 10, actors::Action::Stand);

        // If the dealer's card is good, hit at 16 or less.
        players_tests::check_action_from_cards::<AutoPlayer>((10, 6), 10, actors::Action::Hit);

        // If the dealer's card is bad, stand at 12 or more.
        players_tests::check_action_from_cards::<AutoPlayer>((10, 2), 4, actors::Action::Stand);

        // If the dealer's card is bad, hit at 11 or less.
        players_tests::check_action_from_cards::<AutoPlayer>((8, 3), 4, actors::Action::Hit);

        // If the dealer's card is fair, stand at 13 or more.
        players_tests::check_action_from_cards::<AutoPlayer>((10, 3), 2, actors::Action::Stand);

        // If the dealer's card is fair, hit at 12 or less.
        players_tests::check_action_from_cards::<AutoPlayer>((10, 2), 2, actors::Action::Hit);
    }
}
