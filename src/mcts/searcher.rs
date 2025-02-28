use std::time::Instant;

use super::{state::State, tree::Tree};
use crate::game_state::GameState;

pub struct Searcher {
    tree: Tree<GameState>,
}

impl Searcher {
    pub fn search(&mut self, state: GameState, player: usize, time: u128) {
        let root_id = self.tree.add_state(state.clone(), None);

        let mut i = 0;
        let started = Instant::now();

        loop {
            if i % 2048 == 0 && started.elapsed().as_millis() > time {
                break;
            }

            let state = state.randomize(player);

            let node_id = self.tree.select(root_id);
            let node_id = self.tree.expand(node_id);
            let reward = self.simulate(node_id);
            self.backpropagate(reward, node_id);

            i += 1;
        }
    }

    pub fn simulate(&self, node_id: usize) -> f32 {
        let mut state = self.tree.get_state(node_id);
        while !state.is_terminal() {
            let action = state.possible_actions().pop().unwrap();
            state.apply_action(action);
        }

        todo!()
    }

    fn backpropagate(&mut self, reward: f32, node_id: usize) {}
}
