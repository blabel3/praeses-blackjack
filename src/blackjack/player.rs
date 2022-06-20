use crate::cards;

pub enum PlayerType {
    HumanPlayer,
    AutoPlayer, // For computer-controlled additional players.
    Dealer
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

    pub fn show_hand(&self) {
        match self.player_type {
            PlayerType::HumanPlayer | PlayerType::AutoPlayer => print!("Cards: {}", &self.hand[0]),
            PlayerType::Dealer => print!("Dealer's Cards: **"),
        }

        for card in &self.hand[1..] {
            print!(", {}", card)
        }
        println!("")
    }
}