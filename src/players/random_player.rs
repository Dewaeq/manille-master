use ismcts::{action_list::ActionList, state::State};

use crate::{action::Action, inference::Inference, round::Round};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn decide(&mut self, round: Round, _inference: &Inference) -> Action {
        round.possible_actions().pop_random().unwrap()
    }
}
