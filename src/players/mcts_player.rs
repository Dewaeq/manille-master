use crate::{action::Action, mcts::searcher::Searcher, round::Round};

use super::Player;

pub struct MctsPlayer {
    index: usize,
    searcher: Searcher<Round>,
    search_time: u128,
}

impl Player for MctsPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn decide(&mut self, round: Round) -> Action {
        self.searcher.search(&round, self.search_time)
    }
}

impl MctsPlayer {
    pub fn set_search_time(mut self, time: u128) -> Self {
        self.search_time = time;
        self
    }
}

impl Default for MctsPlayer {
    fn default() -> Self {
        MctsPlayer {
            index: 0,
            searcher: Searcher::new(),
            search_time: 500,
        }
    }
}
