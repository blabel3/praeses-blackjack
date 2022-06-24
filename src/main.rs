use praeses_blackjack::blackjack;

use clap::Parser;

/// Program to play Blackjack
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of real players in the game
    #[clap(short = 'h', long, value_parser, default_value_t = 1)]
    human_players: u32,

    /// Number of bot players in the game
    #[clap(short = 'b', long, value_parser, default_value_t = 0)]
    bot_players: u32,

    /// Number of decks to use in the game
    #[clap(short = 'd', long, value_parser, default_value_t = 6)]
    num_decks: u32,

    /// Payout ratio for the game
    #[clap(short, long, value_parser, default_value_t = 3.0/2.0)]
    payout_ratio: f64,
}

fn main() {
    let args = Args::parse();

    let options = blackjack::GameOptions {
        num_players: args.human_players,
        num_decks: args.num_decks,
        payout_ratio: args.payout_ratio,
    };

    blackjack::play_blackjack::<blackjack::player::Dealer>(options);
}
