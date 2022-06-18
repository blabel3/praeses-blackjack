
use praeses_blackjack::blackjack;
use praeses_blackjack::cards;

fn main() {

    

    println!("Hello, world!");

    let standard_deck = cards::Card::standard_deck();
    println!("{:#?}", standard_deck);
}
