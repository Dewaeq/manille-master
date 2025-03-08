use crate::{card::Card, suit::Suit};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    /// None means play without trump
    PickTrump(Option<Suit>),
    PlayCard(Card),
}
