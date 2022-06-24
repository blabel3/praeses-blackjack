use praeses_blackjack::blackjack;
use praeses_blackjack::blackjack::actors::dealers;

use clap::Parser;

/// Program to play Blackjack
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of real players in the game
    #[clap(short = 'h', long, value_parser, default_value_t = 1)]
    human_players: u32,

    /// If included, will add a bot player to the game.
    #[clap(short = 'r', long, value_parser, default_value_t = false)]
    robot_player: bool,

    /// Number of decks to use in the game
    #[clap(short = 'd', long, value_parser, default_value_t = 6)]
    num_decks: u32,

    /// Initial buy-in for betting (set to 0 to disable betting)
    #[clap(short = 'b', long, value_parser, default_value_t = 500)]
    betting_buyin: u32,

    /// Payout ratio for the game
    #[clap(short, long, value_parser, default_value_t = 3.0/2.0)]
    payout_ratio: f64,
}

fn main() {
    let args = Args::parse();

    let options = blackjack::GameOptions {
        num_players: args.human_players,
        bot_player: args.robot_player,
        num_decks: args.num_decks,
        betting_buyin: args.betting_buyin,
        payout_ratio: args.payout_ratio,
    };

    blackjack::play_blackjack::<dealers::StandardDealer>(options);
}
