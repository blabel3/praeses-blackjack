use crate::cards;

pub enum PlayerType {
    HumanPlayer,
    AutoPlayer, // For computer-controlled additional players.
    Dealer
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
    pub hand: Vec<cards::Card>
}

impl Player {
    pub fn new() -> Player {
        Player {
            player_type: PlayerType::HumanPlayer,
            hand: Vec::new()
        }
    }

    pub fn new_dealer() -> Player {
        Player {
            player_type: PlayerType::Dealer,
            hand: Vec::new()
        }
    }

    pub fn get_hand_value(&self) -> u32 {
        let mut values = self.hand.iter().map(|&card| card.value());
        let sum = values.clone().sum();
        if sum <= 10 && values.any(|x| x == 1) {
            sum + 10
        } else {
            sum
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
            println!("     (value: {})", self.get_hand_value());
        } else {
            println!("");
        }
    }
}