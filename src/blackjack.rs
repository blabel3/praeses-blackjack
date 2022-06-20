mod player;

use std::cmp;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::cards;

struct GameOptions {
    num_players: u32,
    num_decks: u32,
    betting_ratio: f64,
}

struct ReadyGame {
    players: Vec<player::Player>,
    dealer: player::Player,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32
}

struct InProgressGame {
    players: Vec<player::Player>,
    dealer: player::Player,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32
}

impl ReadyGame {
    pub fn new(options: GameOptions) -> ReadyGame {

        let mut players: Vec<player::Player> = Vec::new();

        for _ in 0..options.num_players {
            players.push(player::Player::new())
        }

        let mut deck: Vec<cards::Card> = Vec::new();
        let standard_deck = cards::standard_deck();

        let mut rng = thread_rng();

        for _ in 0..options.num_decks {
            let mut individual_deck = standard_deck;
            individual_deck.shuffle(&mut rng);
            deck.extend_from_slice(&individual_deck);
        }

        ReadyGame {
            players,
            dealer: player::Player::new_dealer(),
            deck,
            betting_ratio: options.betting_ratio,
            reshuffle_at: get_reshuffle_number(&options.num_decks) 
        }
    }

    pub fn deal_hands(mut self) -> InProgressGame {

        for _ in 0..2 {
            for player in &mut self.players {
                player.hand.push(self.deck.pop().unwrap());
            }
            self.dealer.hand.push(self.deck.pop().unwrap());
        }

        InProgressGame {
            players: self.players,
            dealer: self.dealer,
            deck: self.deck,
            betting_ratio: self.betting_ratio,
            reshuffle_at: self.reshuffle_at
        }
    }
}

impl InProgressGame {
    pub fn display_hands(&self) {
        self.dealer.show_hand();
        for player in &self.players {
            player.show_hand();
        }
    }
}

fn get_reshuffle_number(num_decks: &u32) -> u32 {
    let deck_card_count = u32::try_from(cards::STANDARD_DECK_COUNT).unwrap();
    cmp::max(40, num_decks * deck_card_count / 5)
}





// Create game w/ game options

// Play round 

// Optionally continue playing rounds (add/drop players)

pub fn play_blackjack() {

    let options = GameOptions {
        num_players: 1, 
        num_decks: 1,
        betting_ratio: 1.5
    };

    let game = ReadyGame::new(options);

    loop {
        let round = game.deal_hands();

        round.display_hands();



        break;
    }

}



