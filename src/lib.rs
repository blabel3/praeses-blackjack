//! # Praeses Blackjack
//!
//! `praeses_blackjack` is a crate for playing a game of blackjack made in a week for a Praeses job interview.

pub mod blackjack;
pub mod cards;

pub use crate::blackjack::actors::dealers::Dealer;
pub use crate::blackjack::actors::players::Player;
