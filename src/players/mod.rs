use crate::{
    card::{Card, Cards},
    game::Game,
};

pub mod greedy_player;
pub mod random_player;

pub type PlayerVec = Vec<Box<dyn Player>>;

pub trait Player {
    fn boxed() -> Box<Self>
    where
        Self: Sized + Default,
    {
        Box::new(Self::default())
    }

    fn set_index(&mut self, index: usize);

    fn cards(&self) -> Cards;

    fn cards_mut(&mut self) -> &mut Cards;

    fn decide(&self, game: &Game) -> Card;

    fn set_cards(&mut self, cards: Cards) {
        *self.cards_mut() = cards;
    }
    fn toggle_card(&mut self, index: u64) {
        let cards = self.cards_mut();
        cards.data ^= 1 << index;
    }
}
