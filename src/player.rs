use crate::{
    card::{Card, Cards},
    game::Game,
};

pub trait Player {
    fn set_cards(&mut self, cards: Cards);

    fn toggle_card(&mut self, index: u64);

    fn cards(&self) -> Cards;

    fn decide(&self, game: &Game) -> Card;
}
