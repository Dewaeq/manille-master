use crate::{action::Action, round::Round};

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

    fn decide(&mut self, round: Round) -> Action;
}
