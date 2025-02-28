use crate::{action::Action, game_state::GameState, mcts::searcher::Searcher, stack::Stack};

use super::Player;

pub struct MctsPlayer {
    cards: Stack,
    index: usize,
    searcher: Searcher,
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
        self.searcher.search(&state, self.index, 5000)
    }

    //fn pick_trump(&self, state: GameState) -> Option<Suite> {
    //    todo!()
    //}
}

impl Default for MctsPlayer {
    fn default() -> Self {
        MctsPlayer {
            cards: Stack::default(),
            index: 0,
            searcher: Searcher::new(),
        }
    }
}
