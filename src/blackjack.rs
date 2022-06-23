mod player;

use std::cmp;
use std::cmp::Ordering;

use crate::cards;

pub struct GameOptions {
    pub num_players: u32,
    pub num_decks: u32,
    pub betting_ratio: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum RoundResult {
    Win,
    Lose,
    Standoff,
}

type PlayerResult = (Box<dyn player::BlackjackPlayer>, RoundResult);

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

impl<'a, D: 'a> InProgressGame<D>
where
    D: player::BlackjackDealer,
{
    pub fn display_hands(&self) {
        self.dealer.show_hand();
        for player in &self.players {
            player.show_hand();
        }
    }

    fn handle_naturals(self) -> Vec<PlayerResult> {
        let mut game_over_results: Vec<PlayerResult> = Vec::new();
        let dealer_has_natural = hand_is_natural(&self.dealer.get_hand()[..]);

        if dealer_has_natural {
            self.dealer.show_true_hand();
            println!("Dealer has blackjack!");
            for player in self.players {
                let player_has_natural = hand_is_natural(&player.get_hand()[..]);
                if player_has_natural {
                    game_over_results.push((player, RoundResult::Standoff));
                } else {
                    game_over_results.push((player, RoundResult::Lose));
                }
            }
            return game_over_results;
        } else {
            let all_players_have_blackjack = &self.players[..]
                .into_iter()
                .all(|player| hand_is_natural(&player.get_hand()[..]));
            if *all_players_have_blackjack {
                for player in self.players {
                    game_over_results.push((player, RoundResult::Win));
                }
                return game_over_results;
            }
        }
        self.player_turns()
    }

    fn player_turns(mut self) -> Vec<PlayerResult> {
        for player in &mut self.players {
            println!("---Player's turn!---");
            if hand_is_natural(&player.get_hand()[..]) {
                self.dealer.show_hand();
                player.show_hand();
                println!("Blackjack!");
                continue;
            }
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
        self.check_if_all_players_finished()
    }

    fn check_if_all_players_finished(self) -> Vec<PlayerResult> {
        let all_done: bool = self.players[..].into_iter().all(|player| {
            hand_is_bust(&player.get_hand()[..]) || hand_is_natural(&player.get_hand()[..])
        });

        // Needed to maintain player order and look normal, instead of using partition above.
        if all_done {
            let mut game_over_results: Vec<PlayerResult> = Vec::new();
            for player in self.players {
                if hand_is_natural(&player.get_hand()[..]) {
                    game_over_results.push((player, RoundResult::Win))
                } else {
                    game_over_results.push((player, RoundResult::Lose))
                }
            }
            return game_over_results;
        }
        self.dealer_turn()
    }

    fn dealer_turn(mut self) -> Vec<PlayerResult> {
        println!("---Dealer's turn!---");
        loop {
            self.dealer.show_true_hand();
            if hand_is_bust(&self.dealer.get_hand()[..]) {
                println!("Dealer goes bust!");
                let mut game_over_results: Vec<PlayerResult> = Vec::new();
                for player in self.players {
                    if hand_is_bust(&player.get_hand()) {
                        game_over_results.push((player, RoundResult::Lose))
                    } else {
                        game_over_results.push((player, RoundResult::Win))
                    }
                }
                return game_over_results;
            }
            let turn_over = self.dealer.take_turn(&mut self.deck);
            if turn_over {
                break;
            }
        }

        self.complete_round()
    }

    fn complete_round(self) -> Vec<PlayerResult> {
        let mut game_over_results: Vec<PlayerResult> = Vec::new();

        for player in self.players {
            if hand_is_natural(&player.get_hand()[..]) {
                game_over_results.push((player, RoundResult::Win));
                continue;
            }

            match get_hand_value(&player.get_hand()[..])
                .cmp(&get_hand_value(&self.dealer.get_hand()[..]))
            {
                Ordering::Less => game_over_results.push((player, RoundResult::Lose)),
                Ordering::Greater => game_over_results.push((player, RoundResult::Win)),
                Ordering::Equal => game_over_results.push((player, RoundResult::Standoff)),
            }
        }
        game_over_results
    }

    pub fn play_round(self) -> Vec<PlayerResult> {
        self.handle_naturals()
        //if let Some(natural_results) = self.handle_naturals() {
        //    return natural_results;
        //}
        //
        //for player in &mut self.players {
        //    println!("---Player's turn!---");
        //    if hand_is_natural(&player.get_hand()[..]) {
        //        self.dealer.show_hand();
        //        player.show_hand();
        //        println!("Blackjack!");
        //        continue;
        //    }
        //    loop {
        //        self.dealer.show_hand();
        //        player.show_hand(); //# compared to show hands
        //
        //        if hand_is_bust(&player.get_hand()[..]) {
        //            println!("Bust!");
        //            break;
        //        }
        //
        //        let turn_over = player.take_turn(&mut self.deck);
        //        if turn_over {
        //            break;
        //        }
        //
        //        println!("")
        //    }
        //}
        //
        //if let Some(automatic_results) = self.check_if_all_players_finished() {
        //    return automatic_results;
        //}
        //
        //println!("---Dealer's turn!---");
        //loop {
        //    self.dealer.show_true_hand();
        //
        //    if hand_is_bust(&self.dealer.get_hand()[..]) {
        //        println!("Dealer goes bust!");
        //        let mut game_over_results: Vec<PlayerResult> = Vec::new();
        //        for player in self.players {
        //            if hand_is_bust(&player.get_hand()){
        //                game_over_results.push((player, RoundResult::Lose))
        //            } else {
        //                game_over_results.push((player, RoundResult::Win))
        //            }
        //        }
        //        return game_over_results;
        //    }
        //
        //    let turn_over = self.dealer.take_turn(&mut self.deck);
        //
        //    if turn_over {
        //        break;
        //    }
        //}
        //
        //// Finish round
        //self.complete_round()
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
        let round = game.deal_hands();

        let round_results = round.play_round();

        for (_player, result) in round_results {
            //player.show_hand();
            println!("{:?}", result);
        }

        //println!("{:#?}", round_results);

        // Optionally continue playing rounds (and add/drop players?)

        break;
    }
}
