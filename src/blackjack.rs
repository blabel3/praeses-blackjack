mod player;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;
use std::cmp::Ordering;

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

// Round Results output with who wins/loses?

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

    pub fn handle_blackjacks(&mut self) -> bool {
        let (mut players_w_bj, mut players_wo_bj): (Vec<_>, Vec<_>) = self.players
            .as_slice()
            .into_iter()
            .partition(|player| hand_is_natural(&player.get_hand()[..]));
            
        if hand_is_natural(&self.dealer.get_hand()[..]) {
            self.dealer.show_true_hand();
            println!("Dealer has blackjack!");
            for player in &mut players_w_bj {
                player.show_hand();
                println!("Standoff!");
                // Handle bets
            }
            for player in &mut players_wo_bj {
                player.show_hand();
                println!("You lost...");
                // Handle bets
            }
            return true;
        } else {
            if players_w_bj.len() > 0 {
                self.dealer.show_true_hand();
                for player in &mut players_w_bj {
                    player.show_hand();
                    println!("Blackjack! You win");
                    // Handle bets
                }
                return true;
            }
        }

        return false;
    }

    // Combine these two functions?
    pub fn handle_player_action(action: player::Action, 
        player: &mut Box<dyn BlackjackPlayer>,
        deck: &mut Vec<cards::Card>) -> bool 
    {
        match action {
            player::Action::Hit => {
                let deal = deck.pop().unwrap();
                println!("NEW CARD: {}", deal);
                player.recieve_card(deal);
                false
            }
            player::Action::Stand => true,
        }
    }

    pub fn handle_dealer_action(&mut self, action: player::Action) -> bool 
    {
        match action {
            player::Action::Hit => {
                let deal = self.deck.pop().unwrap();
                println!("Hit! NEW CARD: {}", deal);
                self.dealer.recieve_card(deal);
                false
            }
            player::Action::Stand => true,
        }
    }

    pub fn play_round(&mut self) {

        let round_over = self.handle_blackjacks();

        if round_over {
            return;
        }

        for player in &mut self.players {
            loop {
                self.dealer.show_hand();
                player.show_hand(); //# compared to show hands

                if hand_is_bust(&player.get_hand()[..]) {
                    println!("Bust!");
                    break;
                }

                let action = player.get_action();

                if Self::handle_player_action(action, player, &mut self.deck) {
                    break;
                }
                println!("")
            }
        }

        let standing_players: Vec<_> = self.players
            .as_slice()
            .into_iter()
            .filter(|player| !hand_is_bust(&player.get_hand()[..])).collect();

        if standing_players.len() == 0 {
            println!("House wins!");
            return;
        }

        println!("---Dealer's turn!---");

        loop {
            self.dealer.show_true_hand();
            //player.show_hand(); //# compared to show hands

            if hand_is_bust(&self.dealer.get_hand()[..]) {
                println!("Dealer goes bust!");
                self.dealer.hand.clear();
                let winning_players: Vec<_> = self.players
                    .as_slice()
                    .into_iter()
                    .filter(|player| !hand_is_bust(&player.get_hand()[..])).collect();
    
                for player in winning_players {
                    player.show_hand();
                    println!("You win!");
                    // Handle bet
                }
                return;
            }

            let action = self.dealer.get_action();

            if self.handle_dealer_action(action) {
                break;
            }
        }

        let mut standing_players: Vec<_> = self.players
            .as_slice()
            .into_iter()
            .filter(|player| !hand_is_bust(&player.get_hand()[..])).collect();

        //let (mut _bust_players, mut alive_players): (Vec<_>, Vec<_>) = self.players
        //    .as_slice()
        //    .into_iter()
        //    .partition(|player| hand_is_bust(&player.get_hand()[..]));

        for player in &mut standing_players {
            player.show_hand();
            match get_hand_value(&player.get_hand()[..])
                .cmp(&get_hand_value(&self.dealer.get_hand()[..])) {
                Ordering::Less => {
                    //player.show_hand();
                    println!("You lose...");
                },
                Ordering::Greater => {
                    //player.show_hand();
                    println!("You win!");
                },
                Ordering::Equal => {
                    //player.show_hand();
                    println!("Stand-off.");
                },
            }
        }

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

pub fn hand_is_natural(hand: &[cards::Card]) -> bool {
    get_hand_value(&hand) == 21
}

pub fn hand_is_bust(hand: &[cards::Card]) -> bool {
    get_hand_value(&hand) > 21
}


pub fn play_blackjack(options: GameOptions) {
    

    let game = ReadyGame::new(options);

    loop {
        let mut round = game.deal_hands();

        round.play_round();

        // Optionally continue playing rounds (and add/drop players?)

        break;
    }
}
