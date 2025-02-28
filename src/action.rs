use crate::{card::Card, suite::Suite};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    /// None means play without trump
    PickTrump(Option<Suite>),
    PlayCard(Card),
}
