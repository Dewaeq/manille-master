use crate::{
    action::Action,
    mcts::{action_list::ActionList, state::State},
    round::Round,
};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer {
    index: usize,
}

impl Player for RandomPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn decide(&mut self, round: Round) -> Action {
        round.possible_actions().pop_random().unwrap()
    }
}
