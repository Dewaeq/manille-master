use crate::{
    action::Action,
    mcts::{action_list::ActionList, state::State},
    round::Round,
};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn decide(&mut self, round: Round) -> Action {
        round.possible_actions().pop_random().unwrap()
    }
}
