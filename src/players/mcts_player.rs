use crate::{action::Action, game_state::GameState, mcts::searcher::Searcher, stack::Stack};

use super::Player;

pub struct MctsPlayer {
    cards: Stack,
    index: usize,
    searcher: Searcher,
    search_time: u128,
}

impl Player for MctsPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn cards(&self) -> Stack {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Stack {
        &mut self.cards
    }

    fn decide(&mut self, state: GameState) -> Action {
        self.searcher.search(&state, self.index, self.search_time)
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
            cards: Stack::default(),
            index: 0,
            searcher: Searcher::new(),
            search_time: 500,
        }
    }
}
