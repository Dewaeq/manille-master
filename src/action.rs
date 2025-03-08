use std::fmt::Display;

use crate::{card::Card, suit::Suit};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    /// None means play without trump
    PickTrump(Option<Suit>),
    PlayCard(Card),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Action::PickTrump(Some(suit)) => write!(f, "{suit}"),
            Action::PickTrump(None) => write!(f, "None"),
            Action::PlayCard(card) => write!(f, "{card}"),
        }
    }
}
