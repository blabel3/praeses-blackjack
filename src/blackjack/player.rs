use crate::cards;

enum PlayerType {
    HumanPlayer,
    AutoPlayer, // For computer-controlled additional players.
    Dealer
}

pub struct Player {
    playerType: PlayerType,
    hand: Vec<cards::Card>
}