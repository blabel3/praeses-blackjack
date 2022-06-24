//! Blackjack game functionality.

pub mod player;

use std::cmp;
use std::cmp::Ordering;
use std::fmt;

use crate::cards;

/// Options for running a game of blackjack.
pub struct GameOptions {
    /// How many players are at the table
    pub num_players: u32,
    /// How many decks are used to create the deck (most popular is six for a 312 card game).
    pub num_decks: u32,
    /// Payout for winning in blackjack, usually 3:2 or 6:5.
    /// Higher is better for the players, lower is better for the house.
    pub payout_ratio: f64,
}

#[derive(Debug, Copy, Clone)]
enum PlayerRoundResult {
    Win,
    Lose,
    Standoff,
}

impl fmt::Display for PlayerRoundResult {
    /// Shows a player's win/lose condition in a more human-readable way.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerRoundResult::Win => write!(f, "You win! Congratulations! :)"),
            PlayerRoundResult::Lose => write!(f, "You lose, sorry. Thanks for playing!"),
            PlayerRoundResult::Standoff => write!(f, "It's a stand-off!"),
        }
    }
}

type PlayerResult = (Box<dyn player::BlackjackPlayer>, PlayerRoundResult);

type RoundResult = Vec<PlayerResult>;

enum IntermediateRoundResult<D: player::BlackjackDealer> {
    Finished {
        results: RoundResult,
        leftover_deck: cards::Deck,
    },
    Unfinished(InProgressGame<D>),
}

struct ReadyGame<D: player::BlackjackDealer> {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: D,
    deck: cards::Deck,
}

struct InProgressGame<D: player::BlackjackDealer> {
    players: Vec<Box<dyn player::BlackjackPlayer>>,
    dealer: D,
    deck: cards::Deck,
}

impl<D> ReadyGame<D>
where
    D: player::BlackjackDealer,
{
    fn new(options: &GameOptions) -> ReadyGame<D> {
        let mut players: Vec<Box<dyn player::BlackjackPlayer>> = Vec::new();

        for _ in 0..options.num_players {
            // Will change with multiplayer and such -- We will have to call new on different players!
            players.push(Box::new(player::HumanPlayer::new()));
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
}

impl<D> InProgressGame<D>
where
    D: player::BlackjackDealer,
{
    fn handle_naturals(self) -> IntermediateRoundResult<D> {
        let mut round_results: RoundResult = Vec::new();
        let dealer_has_natural = hand_is_natural(self.dealer.get_hand_slice());

        if dealer_has_natural {
            self.dealer.show_true_hand();
            println!("Dealer has blackjack!");
            for player in self.players {
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
                for player in self.players {
                    round_results.push((player, PlayerRoundResult::Win));
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
            println!("---Player's turn!---");
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
                let turn_over = player.take_turn(&mut self.deck);
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
                    round_results.push((player, PlayerRoundResult::Win))
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

/// For a slice of cards, return the value of the hand (properly handling Aces)
pub fn get_hand_value(hand: &[cards::Card]) -> u32 {
    let values: Vec<u32> = hand.iter().map(|card| card_value(card)).collect();
    let sum = values.iter().sum();
    if sum <= 11 && values.iter().any(|&x| x == 1) {
        sum + 10
    } else {
        sum
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
fn settle_round(round_results: RoundResult) {
    println!("");
    for (_player, result) in round_results {
        //player.show_hand();
        println!("{}", result);
    }
}

/// Gets the point at which the deck needs to be reshuffled. Basically acts like the
/// plastic card in a deck in a casino--if the deck length is below this number
/// then we need to get a new full reshuffled deck for the next game.__rust_force_expr!
///
/// # Arguments
///
/// * `num_decks` - number of decks used to create the deck for the game. Should be same
/// value that's passed into `cards::create_multideck(num_decks)`
fn get_reshuffle_number(num_decks: &u32) -> u32 {
    let deck_card_count = u32::try_from(cards::STANDARD_DECK_COUNT).unwrap();
    cmp::max(40, num_decks * deck_card_count / 5)
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
/// num_decks: 6,
/// payout_ratio: 1.5,
/// };
///
/// // blackjack::play_blackjack::<blackjack::player::Dealer>(options);
/// ```
pub fn play_blackjack<D>(options: GameOptions)
where
    D: player::BlackjackDealer,
{
    let game: ReadyGame<D> = ReadyGame::new(&options);

    let _reshuffle_at = get_reshuffle_number(&options.num_decks);
    let _betting_ratio = &options.payout_ratio;

    loop {
        let round = game.deal_hands();

        let finished_round = round.play_round();

        let (round_results, _leftover_deck) = finished_round;

        settle_round(round_results);

        // Optionally continue playing rounds (and add/drop players?)

        break;
    }
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
