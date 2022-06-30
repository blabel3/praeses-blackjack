//! Blackjack game functionality.

pub mod actors;

use std::cmp;
use std::cmp::Ordering;
use std::{fmt, io};

use crate::blackjack::actors::dealers::Dealer;
use crate::blackjack::actors::players::{self, Player};
use crate::cards;

/// Options for running a game of blackjack.
pub struct GameOptions {
    /// How many players are at the table
    pub num_players: u32,
    /// Whether or not to play alongside a bot player.
    pub bot_player: bool,
    /// How many decks are used to create the deck (most popular is six for a 312 card game).
    pub num_decks: u32,
    /// How much money to give players to start with (and if/when they run out).
    pub betting_buyin: u32,
    /// Payout for winning in blackjack, usually 3:2 or 6:5.
    /// Higher is better for the players, lower is better for the house.
    pub payout_ratio: f64,
}

/// Possible results for each player each round.
#[derive(Debug, Copy, Clone)]
pub enum PlayerRoundResult {
    /// Natural or Blackjack is when the player has 21 in the first two cards. (But if the dealer matches then it's a standoff)
    Natural,
    /// If they have more than the dealer at the end of the round or the dealer goes bust while they stand.
    Win,
    /// If they go bust or if the dealer has more than them at the end of the round.
    Lose,
    /// If their value and the dealer's are exactly equal at the end of the round.
    Standoff,
}

impl fmt::Display for PlayerRoundResult {
    /// Shows a player's win/lose condition in a more human-readable way.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerRoundResult::Natural => write!(f, "Blackjack! Wow, lucky!"),
            PlayerRoundResult::Win => write!(f, "You win! Congratulations!"),
            PlayerRoundResult::Lose => write!(f, "Sorry, you lose."),
            PlayerRoundResult::Standoff => write!(f, "It's a stand-off!"),
        }
    }
}

type PlayerResult = (Box<dyn Player>, PlayerRoundResult);

type RoundResult = Vec<PlayerResult>;

enum IntermediateRoundResult<D: Dealer> {
    Finished {
        results: RoundResult,
        leftover_deck: cards::Deck,
    },
    Unfinished(InProgressGame<D>),
}

struct ReadyGame<D: Dealer> {
    players: Vec<Box<dyn Player>>,
    dealer: D,
    deck: cards::Deck,
}

struct InProgressGame<D: Dealer> {
    players: Vec<Box<dyn Player>>,
    dealer: D,
    deck: cards::Deck,
}

impl<D> ReadyGame<D>
where
    D: Dealer,
{
    fn new(options: &GameOptions) -> ReadyGame<D> {
        let mut players: Vec<Box<dyn players::Player>> = Vec::new();

        if options.bot_player {
            players.push(Box::new(players::AutoPlayer::new(options.betting_buyin)));
        }

        for _ in 0..options.num_players {
            // Will change with multiplayer and such -- We will have to call new on different players!
            players.push(Box::new(players::HumanPlayer::new(options.betting_buyin)));
        }

        let mut deck = cards::create_multideck(options.num_decks);
        cards::shuffle_deck(&mut deck);

        ReadyGame {
            players,
            dealer: D::new(),
            deck,
        }
    }

    fn deal_hands(mut self) -> InProgressGame<D> {
        for player in &mut self.players {
            player.set_bet();
        }

        println!("");

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
        }
    }

    fn from_previous_round(
        players: Vec<Box<dyn Player>>,
        leftover_deck: cards::Deck,
        options: &GameOptions,
    ) -> ReadyGame<D> {
        let mut ready_players: Vec<Box<dyn Player>> = Vec::new();
        for mut player in players {
            player.buy_in_if_broke(options.betting_buyin);
            ready_players.push(player);
        }

        let mut deck: cards::Deck;
        if leftover_deck.len() > get_reshuffle_number(options.num_decks).try_into().unwrap() {
            deck = leftover_deck;
        } else {
            println!("Reshuffling deck...\n");
            deck = cards::create_multideck(options.num_decks);
            cards::shuffle_deck(&mut deck);
        };

        ReadyGame {
            players: ready_players,
            dealer: D::new(),
            deck,
        }
    }
}

impl<D> InProgressGame<D>
where
    D: Dealer,
{
    fn handle_naturals(self) -> IntermediateRoundResult<D> {
        let mut round_results: RoundResult = Vec::new();
        let dealer_has_natural = hand_is_natural(self.dealer.get_hand_slice());

        if dealer_has_natural {
            self.dealer.show_true_hand();
            println!("Dealer has blackjack!");
            for player in self.players {
                player.show_hand();
                let player_has_natural = hand_is_natural(player.get_hand_slice());
                if player_has_natural {
                    round_results.push((player, PlayerRoundResult::Standoff));
                } else {
                    round_results.push((player, PlayerRoundResult::Lose));
                }
            }
            return IntermediateRoundResult::Finished {
                results: round_results,
                leftover_deck: self.deck,
            };
        } else {
            let all_players_have_blackjack = &self.players[..]
                .into_iter()
                .all(|player| hand_is_natural(player.get_hand_slice()));
            if *all_players_have_blackjack {
                self.dealer.show_true_hand();
                for player in self.players {
                    player.show_hand();
                    round_results.push((player, PlayerRoundResult::Natural));
                }
                return IntermediateRoundResult::Finished {
                    results: round_results,
                    leftover_deck: self.deck,
                };
            }
        }
        IntermediateRoundResult::Unfinished(self)
    }

    fn player_turns(&mut self) {
        for player in &mut self.players {
            println!("---{}'s turn!---", player.get_name());
            // If they had blackjack, they do not take a turn.
            if hand_is_natural(player.get_hand_slice()) {
                self.dealer.show_hand();
                player.show_hand();
                println!("Blackjack!");
                continue;
            }

            loop {
                self.dealer.show_hand();
                player.show_hand(); //# compared to show hands
                if hand_is_bust(player.get_hand_slice()) {
                    println!("Bust!");
                    break;
                }
                let turn_over = player.take_turn(&mut self.deck, &self.dealer.get_hand_slice()[1]);
                if turn_over {
                    break;
                }
                println!("")
            }
        }
    }

    fn check_if_all_players_finished(self) -> IntermediateRoundResult<D> {
        let all_done: bool = self.players[..].into_iter().all(|player| {
            hand_is_bust(player.get_hand_slice()) || hand_is_natural(player.get_hand_slice())
        });

        if all_done {
            let mut round_results: RoundResult = Vec::new();
            for player in self.players {
                if hand_is_natural(player.get_hand_slice()) {
                    round_results.push((player, PlayerRoundResult::Natural))
                } else {
                    round_results.push((player, PlayerRoundResult::Lose))
                }
            }
            return IntermediateRoundResult::Finished {
                results: round_results,
                leftover_deck: self.deck,
            };
        }
        IntermediateRoundResult::Unfinished(self)
    }

    fn dealer_turn(mut self) -> IntermediateRoundResult<D> {
        println!("---Dealer's turn!---");
        loop {
            self.dealer.show_true_hand();
            if hand_is_bust(self.dealer.get_hand_slice()) {
                println!("Dealer goes bust!");
                let mut round_results: RoundResult = Vec::new();
                for player in self.players {
                    if hand_is_bust(player.get_hand_slice()) {
                        round_results.push((player, PlayerRoundResult::Lose))
                    } else {
                        round_results.push((player, PlayerRoundResult::Win))
                    }
                }
                return IntermediateRoundResult::Finished {
                    results: round_results,
                    leftover_deck: self.deck,
                };
            }
            let turn_over = self.dealer.take_turn(&mut self.deck);
            if turn_over {
                break;
            }
        }
        return IntermediateRoundResult::Unfinished(self);
    }

    fn complete_round(self) -> (RoundResult, cards::Deck) {
        let mut round_results: RoundResult = Vec::new();

        for player in self.players {
            // If a player had blackjack, they win even if the dealer got to 21 themselves later.
            // If dealer had blackjack, then the game would've ended before this call.
            if hand_is_natural(player.get_hand_slice()) {
                round_results.push((player, PlayerRoundResult::Win));
                continue;
            }

            // If a player is bust then they lose.
            if hand_is_bust(player.get_hand_slice()) {
                round_results.push((player, PlayerRoundResult::Lose));
                continue;
            }

            match get_hand_value(player.get_hand_slice())
                .cmp(&get_hand_value(self.dealer.get_hand_slice()))
            {
                Ordering::Less => round_results.push((player, PlayerRoundResult::Lose)),
                Ordering::Greater => round_results.push((player, PlayerRoundResult::Win)),
                Ordering::Equal => round_results.push((player, PlayerRoundResult::Standoff)),
            }
        }
        (round_results, self.deck)
    }

    fn play_round(mut self) -> (RoundResult, cards::Deck) {
        // Check if anybody has blackjack, and handle it appropriately.
        let natural_results = self.handle_naturals();
        match natural_results {
            IntermediateRoundResult::Finished {
                results,
                leftover_deck,
            } => return (results, leftover_deck),
            IntermediateRoundResult::Unfinished(game) => self = game,
        }

        // Let the players take their turns, and check if the game is over.
        self.player_turns();
        let player_turn_results = self.check_if_all_players_finished();
        match player_turn_results {
            IntermediateRoundResult::Finished {
                results,
                leftover_deck,
            } => return (results, leftover_deck),
            IntermediateRoundResult::Unfinished(game) => self = game,
        }

        // Let the dealer make their turn. Will end if they go bust.
        let dealer_turn_results = self.dealer_turn();
        match dealer_turn_results {
            IntermediateRoundResult::Finished {
                results,
                leftover_deck,
            } => return (results, leftover_deck),
            IntermediateRoundResult::Unfinished(game) => self = game,
        }

        // Check who won, lost, or tied based on the hands of the players (which are all complete)
        self.complete_round()
    }
}

/// Given a card, return it's numeric value in Blackjack.
/// Aces count as 1, and will get the extra 10 if it doesn't make the player go bust
/// when taking their whole hand value into account.
pub fn card_value(card: &cards::Card) -> u32 {
    match card.rank {
        cards::Rank::Ace => 1,
        cards::Rank::Two => 2,
        cards::Rank::Three => 3,
        cards::Rank::Four => 4,
        cards::Rank::Five => 5,
        cards::Rank::Six => 6,
        cards::Rank::Seven => 7,
        cards::Rank::Eight => 8,
        cards::Rank::Nine => 9,
        cards::Rank::Ten => 10,
        cards::Rank::Jack => 10,
        cards::Rank::Queen => 10,
        cards::Rank::King => 10,
    }
}

/// For a slice of cards, get the raw value of the hand (not counting aces potentially as 11)
pub fn get_raw_hand_value(hand: &[cards::Card]) -> u32 {
    let values: Vec<u32> = hand.iter().map(|card| card_value(card)).collect();
    values.iter().sum()
}

/// Return true if the hand has an ace that can be counted as 11.
pub fn is_soft_hand(raw_value: u32, hand: &[cards::Card]) -> bool {
    raw_value <= 11 && hand.iter().any(|&card| card.rank == cards::Rank::Ace)
}

/// For a slice of cards, return the value of the hand (properly handling Aces)
pub fn get_hand_value(hand: &[cards::Card]) -> u32 {
    let raw_value: u32 = get_raw_hand_value(hand);
    if is_soft_hand(raw_value, hand) {
        raw_value + 10
    } else {
        raw_value
    }
}

/// For a slice of cards, return true if the value of the hand is exactly 21 and there are only 2 cards in the hand.
pub fn hand_is_natural(hand: &[cards::Card]) -> bool {
    get_hand_value(&hand) == 21 && hand.len() == 2
}

/// For a slice of cards, return true if the value of the hand is over 21.
pub fn hand_is_bust(hand: &[cards::Card]) -> bool {
    get_hand_value(&hand) > 21
}

/// Settles the round--goes over the results (and bets once those are added)
fn settle_round(round_results: RoundResult, &payout_ratio: &f64) -> Vec<Box<dyn Player>> {
    println!("");
    let mut new_players: Vec<Box<dyn Player>> = Vec::new();
    for (mut player, result) in round_results {
        //player.show_hand();
        player.discard_hand();
        player.handle_round_result(result, payout_ratio);
        new_players.push(player);
    }
    new_players
}

/// Gets the point at which the deck needs to be reshuffled. Basically acts like the
/// plastic card in a deck in a casino--if the deck length is below this number
/// then we need to get a new full reshuffled deck for the next game.__rust_force_expr!
///
/// # Arguments
///
/// * `num_decks` - number of decks used to create the deck for the game. Should be same
/// value that's passed into `cards::create_multideck(num_decks)`
fn get_reshuffle_number(num_decks: u32) -> u32 {
    let deck_card_count = u32::try_from(cards::STANDARD_DECK_COUNT).unwrap();
    cmp::max(40, num_decks * deck_card_count / 5)
}

fn should_play_another_round() -> bool {
    println!("\nPlay another round? [Y/n]");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match &input.to_lowercase()[..] {
            "" | "yes" | "y" => return true,
            "n" | "No" | "q" | "quit" | "e" | "exit" => return false,
            _ => println!(
                "Sorry, what was that? (try yes, no, exit, or the first letters of any of those."
            ),
        }
    }
}

/// From a GameOptions describing the settings of the game, play a full game of blackjack.
/// Takes a dealer type, which is the dealer that the game will use.
///
/// # Example
///
/// ```
/// use praeses_blackjack::blackjack;
///
/// let options = blackjack::GameOptions {
/// num_players: 1,
/// bot_player: false,
/// num_decks: 6,
/// betting_buyin: 500,
/// payout_ratio: 1.5,
/// };
///
/// // blackjack::play_blackjack::<blackjack::player::Dealer>(options);
/// ```
pub fn play_blackjack<D>(options: GameOptions)
where
    D: Dealer,
{
    let mut game: ReadyGame<D> = ReadyGame::new(&options);

    loop {
        let round = game.deal_hands();

        let finished_round = round.play_round();

        let (round_results, leftover_deck) = finished_round;

        let next_players = settle_round(round_results, &options.payout_ratio);

        // Check if they want to play another round.
        // Optionally continue playing rounds (and add/drop players?)
        if should_play_another_round() {
            println!("");
            game = ReadyGame::from_previous_round(next_players, leftover_deck, &options);
        } else {
            break;
        }
    }

    println!("Thanks for playing!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_value_correct() {
        assert_eq!(
            21,
            get_hand_value(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Spade
                }
            ])
        );
        assert_eq!(
            18,
            get_hand_value(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::Seven,
                    suit: cards::Suit::Diamond
                },
                cards::Card {
                    rank: cards::Rank::Jack,
                    suit: cards::Suit::Heart
                }
            ])
        );
        assert_eq!(
            20,
            get_hand_value(&[
                cards::Card {
                    rank: cards::Rank::Queen,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Diamond
                }
            ])
        );
    }

    #[test]
    fn detects_naturals() {
        assert_eq!(
            true,
            hand_is_natural(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Spade
                }
            ])
        );
        assert_eq!(
            false,
            hand_is_natural(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::Seven,
                    suit: cards::Suit::Diamond
                },
                cards::Card {
                    rank: cards::Rank::Three,
                    suit: cards::Suit::Heart
                }
            ])
        );
        assert_eq!(
            false,
            hand_is_natural(&[
                cards::Card {
                    rank: cards::Rank::Queen,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Diamond
                }
            ])
        );
    }

    #[test]
    fn detects_busts() {
        assert_eq!(
            false,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Spade
                }
            ])
        );
        assert_eq!(
            false,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::Seven,
                    suit: cards::Suit::Diamond
                },
                cards::Card {
                    rank: cards::Rank::Four,
                    suit: cards::Suit::Heart
                }
            ])
        );
        assert_eq!(
            true,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Ace,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Diamond
                },
                cards::Card {
                    rank: cards::Rank::Nine,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::Seven,
                    suit: cards::Suit::Heart
                }
            ])
        );
        assert_eq!(
            false,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Queen,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Diamond
                }
            ])
        );
        assert_eq!(
            false,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Two,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::Two,
                    suit: cards::Suit::Diamond
                }
            ])
        );
        assert_eq!(
            true,
            hand_is_bust(&[
                cards::Card {
                    rank: cards::Rank::Queen,
                    suit: cards::Suit::Heart
                },
                cards::Card {
                    rank: cards::Rank::King,
                    suit: cards::Suit::Diamond
                },
                cards::Card {
                    rank: cards::Rank::Nine,
                    suit: cards::Suit::Club
                },
                cards::Card {
                    rank: cards::Rank::Ten,
                    suit: cards::Suit::Diamond
                }
            ])
        );
    }
}
