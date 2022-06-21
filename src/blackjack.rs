mod player;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;

use crate::cards;
use crate::blackjack::player::BlackjackPlayer;

pub struct GameOptions {
    pub num_players: u32,
    pub num_decks: u32,
    pub betting_ratio: f64,
}

struct ReadyGame {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: player::Dealer,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32,
}

struct InProgressGame {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: player::Dealer,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32,
}

impl ReadyGame {
    pub fn new(options: GameOptions) -> ReadyGame {
        let mut players: Vec<Box<dyn player::BlackjackPlayer>> = Vec::new();

        for _ in 0..options.num_players {
            players.push(Box::new(
                player::HumanPlayer::new()
            ))
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
            dealer: player::Dealer::new(),
            deck,
            betting_ratio: options.betting_ratio,
            reshuffle_at: get_reshuffle_number(&options.num_decks),
        }
    }

    pub fn deal_hands(mut self) -> InProgressGame {
        for _ in 0..2 {
            for player in &mut self.players {
                player.recieve_card(self.deck.pop().unwrap());
            }
            self.dealer.recieve_card(self.deck.pop().unwrap());
        }

        InProgressGame {
            players: self.players,
            dealer: self.dealer,
            deck: self.deck,
            betting_ratio: self.betting_ratio,
            reshuffle_at: self.reshuffle_at,
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

    pub fn get_player_action(&self) {}

    pub fn play_round(&mut self) {
        for player in &mut self.players {
            loop {
                self.dealer.show_hand();
                player.show_hand(); //# compared to show hands

                if get_hand_value(&player.get_hand()[..]) > 21 {
                    println!("Bust!");
                    break;
                }

                let action = player.get_action();

                match action {
                    player::Action::Hit => {
                        let deal = self.deck.pop().unwrap();
                        println!("NEW CARD: {}", deal);
                        player.recieve_card(deal);
                    }
                    player::Action::Stand => break,
                }
                println!("")
            }
        }

        if self
            .players
            .iter()
            .all(|player| get_hand_value(&player.get_hand()[..]) > 21)
        {
            println!("House wins!");
            return ();
        }

        println!("---Dealer's turn!---");

        loop {
            self.dealer.show_true_hand();
            //player.show_hand(); //# compared to show hands

            if get_hand_value(&self.dealer.hand[..]) > 21 {
                println!("Dealer goes bust!");
                break;
            }

            let action = self.dealer.get_action();

            match action {
                player::Action::Hit => {
                    let deal = self.deck.pop().unwrap();
                    self.dealer.hand.push(deal);
                }
                player::Action::Stand => break,
            }
        }

        // dealer's turn
    }
}

fn get_reshuffle_number(num_decks: &u32) -> u32 {
    let deck_card_count = u32::try_from(cards::STANDARD_DECK_COUNT).unwrap();
    cmp::max(40, num_decks * deck_card_count / 5)
}

pub fn get_hand_value(hand: &[cards::Card]) -> u32 {
    let values: Vec<u32> = hand.iter().map(|&card| card.value()).collect();
    let sum = values.iter().sum();
    if sum <= 11 && values.iter().any(|&x| x == 1) {
        sum + 10
    } else {
        sum
    }
}

// Create game w/ game options

// Play round

// Optionally continue playing rounds (add/drop players)

pub fn play_blackjack(options: GameOptions) {
    

    let game = ReadyGame::new(options);

    loop {
        let mut round = game.deal_hands();

        round.play_round();

        //round.display_hands();

        break;
    }
}
