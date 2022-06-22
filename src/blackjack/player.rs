use crate::blackjack;
use crate::cards;
use std::io;

#[derive(Debug)]
pub enum Action {
    Hit,
    Stand,
}

impl Action {
    pub fn parse_from_string(input: &str) -> Self {
        let input = &input.to_lowercase()[..];
        match input {
            "hit" | "h" => Self::Hit,
            "stand" | "s" => Self::Stand,
            _ => panic!("WHY BLAKE"),
        }
    }
}

pub trait BlackjackPlayer {
    fn get_hand(&self) -> &Vec<cards::Card>;

    fn show_hand(&self) -> ();

    // Can probably turn new and this into a macro maybe?
    fn recieve_card(&mut self, card: cards::Card) -> ();

    fn get_action(&self) -> Action;

    fn handle_player_action(&mut self, action: Action, deck: &mut Vec<cards::Card>) -> bool {
        match action {
            Action::Hit => {
                let deal = deck.pop().unwrap();
                println!("Hit! NEW CARD: {}", deal);
                self.recieve_card(deal);
                false
            }
            Action::Stand => true,
        }
    }

    fn take_turn(&mut self, deck: &mut Vec<cards::Card>) -> bool {
        let action = self.get_action();
        self.handle_player_action(action, deck)
    }

    // fn new() -> Self {
    //     Self {
    //         hand: Vec::new()
    //     }
    // }
}

pub struct Dealer {
    pub hand: Vec<cards::Card>,
}

// Dealer probably doesn't need to implement this actually...
impl BlackjackPlayer for Dealer {
    fn get_action(&self) -> Action {
        if blackjack::get_hand_value(&self.hand[..]) >= 17 {
            Action::Stand
        } else {
            Action::Hit
        }
    }

    fn get_hand(&self) -> &Vec<cards::Card> {
        &self.hand
    }

    fn show_hand(&self) {
        print!("Dealer's Cards: **");
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!("");
    }

    fn recieve_card(&mut self, card: cards::Card) {
        self.hand.push(card);
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer { hand: Vec::new() }
    }

    pub fn show_true_hand(&self) {
        print!("Dealer's Cards: {}", &self.hand[0]);
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!(
            "     (value: {})",
            blackjack::get_hand_value(&self.hand[..])
        );
    }
}

pub struct HumanPlayer {
    pub hand: Vec<cards::Card>,
}

impl BlackjackPlayer for HumanPlayer {
    fn get_action(&self) -> Action {
        let mut action = String::new();

        println!("hit or stand?   ");

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        action = action.trim().to_string();

        let action: Action = Action::parse_from_string(&action);
        //println!("{:?}", action);
        action
    }

    fn get_hand(&self) -> &Vec<cards::Card> {
        &self.hand
    }

    fn show_hand(&self) {
        print!("Cards: {}", &self.hand[0]);
        for card in &self.hand[1..] {
            print!(", {}", card);
        }
        println!(
            "     (value: {})",
            blackjack::get_hand_value(&self.hand[..])
        );
    }

    //fn display_hand(&self) -> String {
    //    let output: String = String::new();
    //    output.push_str(&self.hand[0]);
    //    //for card in &self.hand[1..] {
    //    //    print!(", {}", card);
    //    //}
    //    putput
    //    //println!("     (value: {})", blackjack::get_hand_value(&self.hand[..]));
    //}

    fn recieve_card(&mut self, card: cards::Card) {
        self.hand.push(card);
    }
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer { hand: Vec::new() }
    }
}
