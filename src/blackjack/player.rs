use crate::cards;
use std::io;

#[derive(Debug)]
pub enum PlayerType {
    HumanPlayer,
    AutoPlayer, // For computer-controlled additional players.
    Dealer,
}

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

impl PlayerType {
    pub fn should_hide_hand_value(&self) -> bool {
        match self {
            Self::HumanPlayer | Self::AutoPlayer => false,
            Self::Dealer => true,
        }
    }
}

pub struct Player {
    player_type: PlayerType,
    pub hand: Vec<cards::Card>,
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

impl Player {
    pub fn new() -> Player {
        Player {
            player_type: PlayerType::HumanPlayer,
            hand: Vec::new(),
        }
    }

    pub fn new_dealer() -> Player {
        Player {
            player_type: PlayerType::Dealer,
            hand: Vec::new(),
        }
    }

    pub fn show_hand(&self) {
        match self.player_type {
            PlayerType::HumanPlayer | PlayerType::AutoPlayer => print!("Cards: {}", &self.hand[0]),
            PlayerType::Dealer => print!("Dealer's Cards: **"),
        }

        for card in &self.hand[1..] {
            print!(", {}", card);
        }

        if !self.player_type.should_hide_hand_value() {
            println!("     (value: {})", get_hand_value(&self.hand[..]));
        } else {
            println!("");
        }
    }

    pub fn show_dealer_hand(&self) {
        match self.player_type {
            PlayerType::HumanPlayer | PlayerType::AutoPlayer => panic!("Nooooo"),
            PlayerType::Dealer => {
                print!("Dealer's Cards: {}", &self.hand[0]);
                for card in &self.hand[1..] {
                    print!(", {}", card);
                }
                println!("     (value: {})", get_hand_value(&self.hand[..]));
            }
        }
    }

    pub fn get_action(&self) -> Action {
        match &self.player_type {
            PlayerType::HumanPlayer => {
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
            PlayerType::AutoPlayer => Action::Stand,
            PlayerType::Dealer => {
                if get_hand_value(&self.hand[..]) >= 17 {
                    Action::Stand
                } else {
                    Action::Hit
                }
            }
        }
    }
}


pub trait BlackjackPlayer {
    fn get_action(&self) -> Action;
}

pub struct Dealer {
    pub hand: Vec<cards::Card>,
}

impl BlackjackPlayer for Dealer {
    fn get_action(&self) -> Action {
        if get_hand_value(&self.hand[..]) >= 17 {
            Action::Stand
        } else {
            Action::Hit
        }
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            hand: Vec::new()
        }
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
}