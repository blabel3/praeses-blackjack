use praeses_blackjack::blackjack;

fn main() {
    let options = blackjack::GameOptions {
        num_players: 1,
        num_decks: 6,
        payout_ratio: 1.5,
    };

    blackjack::play_blackjack(options);

    // let standard_deck = cards::Card::standard_deck();
    // println!("{:#?}", standard_deck);

    // println!("{}", standard_deck[0]);

    //let mut rng = thread_rng();
}
