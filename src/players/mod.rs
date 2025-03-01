use crate::{action::Action, game_state::GameState};

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

    fn decide(&mut self, state: GameState) -> Action;

    //fn pick_trump(&self, staate: GameState) -> Option<Suite>;
}
