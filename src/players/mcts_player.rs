use crate::{
    action::Action, card::Card, game::Game, game_state::GameState, mcts::tree::Tree, stack::Stack,
    suite::Suite,
};

use super::Player;

pub struct MctsPlayer {
    cards: Stack,
    index: usize,
    tree: Tree<GameState>,
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

    fn decide(&self, game: &Game) -> Action {
        todo!()
    }

    fn pick_trump(&self, game: &Game) -> Option<Suite> {
        todo!()
    }
}

impl MctsPlayer {}
