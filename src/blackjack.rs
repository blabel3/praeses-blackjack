mod player;

use crate::cards;

enum GameStatus {
    New,
    Ready,
    InProgress,
    RoundOver,
    Done
}

struct BlackjackGame {
    status: GameStatus,
    players: Vec<player::Player>,
    deck: Vec<cards::Card>
}



