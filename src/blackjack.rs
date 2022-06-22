mod player;

use std::cmp;
use std::cmp::Ordering;

use crate::cards;

pub struct GameOptions {
    pub num_players: u32,
    pub num_decks: u32,
    pub betting_ratio: f64,
}

struct ReadyGame<D: player::BlackjackDealer> {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: D,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32,
}

struct InProgressGame<D: player::BlackjackDealer> {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: D,
    deck: Vec<cards::Card>,
    betting_ratio: f64,
    reshuffle_at: u32,
}

// Round Results output with who wins/loses?

impl<D> ReadyGame<D>
where
    D: player::BlackjackDealer,
{
    pub fn new(options: &GameOptions) -> ReadyGame<D> {
        let mut players: Vec<Box<dyn player::BlackjackPlayer>> = Vec::new();

        for _ in 0..options.num_players {
            // Will change with multiplayer and such.
            players.push(Box::new(player::HumanPlayer::new()));
        }

        let deck = cards::get_shuffled_deck(options.num_decks);

        ReadyGame {
            players,
            dealer: D::new(),
            deck,
            betting_ratio: options.betting_ratio,
            reshuffle_at: get_reshuffle_number(&options.num_decks),
        }
    }

    pub fn deal_hands(mut self) -> InProgressGame<D> {
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

impl<D> InProgressGame<D>
where
    D: player::BlackjackDealer,
{
    pub fn display_hands(&self) {
        self.dealer.show_hand();
        for player in &self.players {
            player.show_hand();
        }
    }

    pub fn handle_blackjacks(players: &[Box<dyn player::BlackjackPlayer>], dealer: &D) -> bool {
        let (mut players_w_bj, mut players_wo_bj): (Vec<_>, Vec<_>) = players
            .into_iter()
            .partition(|player| hand_is_natural(&player.get_hand()[..]));

        //Maybe a pattern match on the tuple instead?

        if hand_is_natural(&dealer.get_hand()[..]) {
            dealer.show_true_hand();
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
                dealer.show_true_hand();
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

    pub fn play_round(&mut self) {
        let round_over = Self::handle_blackjacks(&self.players[..], &self.dealer);

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

                let turn_over = player.take_turn(&mut self.deck);
                if turn_over {
                    break;
                }

                println!("")
            }
        }

        let standing_players: Vec<_> = self
            .players
            .as_slice()
            .into_iter()
            .filter(|player| !hand_is_bust(&player.get_hand()[..]))
            .collect();

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
                // self.dealer.hand.clear();
                let winning_players: Vec<_> = self
                    .players
                    .as_slice()
                    .into_iter()
                    .filter(|player| !hand_is_bust(&player.get_hand()[..]))
                    .collect();

                for player in winning_players {
                    player.show_hand();
                    println!("You win!");
                    // Handle bet
                }
                return;
            }

            let turn_over = self.dealer.take_turn(&mut self.deck);

            if turn_over {
                break;
            }
        }

        let mut standing_players: Vec<_> = self
            .players
            .as_slice()
            .into_iter()
            .filter(|player| !hand_is_bust(&player.get_hand()[..]))
            .collect();

        //let (mut _bust_players, mut alive_players): (Vec<_>, Vec<_>) = self.players
        //    .as_slice()
        //    .into_iter()
        //    .partition(|player| hand_is_bust(&player.get_hand()[..]));

        for player in &mut standing_players {
            player.show_hand();
            match get_hand_value(&player.get_hand()[..])
                .cmp(&get_hand_value(&self.dealer.get_hand()[..]))
            {
                Ordering::Less => {
                    //player.show_hand();
                    println!("You lose...");
                }
                Ordering::Greater => {
                    //player.show_hand();
                    println!("You win!");
                }
                Ordering::Equal => {
                    //player.show_hand();
                    println!("Stand-off.");
                }
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
    let game: ReadyGame<player::Dealer> = ReadyGame::new(&options);

    loop {
        let mut round = game.deal_hands();

        round.play_round();

        // Optionally continue playing rounds (and add/drop players?)

        break;
    }
}
