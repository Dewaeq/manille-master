use std::time::Instant;

use super::{state::State, tree::Tree};
use crate::{action::Action, game_state::GameState, mcts::action_list::ActionList};

pub struct Searcher {
    tree: Tree<GameState>,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher { tree: Tree::new() }
    }

    pub fn search(&mut self, state: &GameState, time: u128) -> Action {
        self.tree.reset();
        let root_id = self.tree.add_node(state, None, None);

        let mut i = 0;
        let started = Instant::now();

        loop {
            if i % 2048 == 0 && started.elapsed().as_millis() > time {
                break;
            }

            let mut state = state.randomize(state.turn());

            let node_id = self.tree.select(root_id, &mut state);
            let node_id = self.tree.expand(node_id, &mut state);
            let reward = self.simulate(node_id, &mut state);
            self.backpropagate(reward, node_id);

            i += 1;
        }

        #[cfg(feature = "debug")]
        {
            self.tree.dbg_actions(root_id, state);
            let elapsed = started.elapsed().as_millis();
            println!("tree size: {:?}", self.tree.size());
            println!(
                "ran {i} simulations in {} ms, thats {} sims/s",
                elapsed,
                i as f32 / elapsed as f32 * 1000.
            );
        }
        self.tree.best_action(root_id, state).unwrap()
    }

    pub fn simulate(&self, node_id: usize, state: &mut GameState) -> f32 {
        let perspective = self.tree.get_edge(node_id).unwrap().actor();

        while !state.is_terminal() {
            let action = state.possible_actions().pop_random().unwrap();
            state.apply_action(action);
        }

        state.reward(perspective)
    }

    fn backpropagate(&mut self, mut reward: f32, node_id: usize) {
        let mut node_id = Some(node_id);

        while let Some(id) = node_id {
            self.tree.update_node(id, reward);
            node_id = self.tree.get_parent_id(id);
            reward = 1. - reward;
        }
    }
}
