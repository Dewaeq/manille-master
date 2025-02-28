use crate::{action::Action, game_state::GameState, stack::Stack, suite::Suite};

pub mod greedy_player;
pub mod mcts_player;
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

    fn cards(&self) -> Stack;

    fn cards_mut(&mut self) -> &mut Stack;

    fn decide(&mut self, state: GameState) -> Action;

    //fn pick_trump(&self, staate: GameState) -> Option<Suite>;

    fn set_cards(&mut self, cards: Stack) {
        *self.cards_mut() = cards;
    }

    fn toggle_card(&mut self, index: u32) {
        let cards = self.cards_mut();
        *cards ^= 1 << index;
    }
}
