use std::{fmt::Debug, time::Instant};

use super::{state::State, tree::Tree};

pub struct Searcher<T: State + Clone> {
    tree: Tree<T>,
}

impl<T: State + Clone> Searcher<T> {
    pub fn new() -> Self {
        Searcher { tree: Tree::new() }
    }

    pub fn search(&mut self, state: &T, time: u128, use_self_determinization: bool) -> T::Action
    where
        T::Action: Debug,
    {
        self.tree.reset();
        let root_id = self.tree.add_node(state, None, None);

        let mut self_determinize = use_self_determinization;
        let mut i = 0;
        let started = Instant::now();

        loop {
            if i % 2048 == 0 {
                if started.elapsed().as_millis() > time {
                    break;
                }

                if self_determinize && started.elapsed().as_millis() > time * 3 / 10 {
                    self_determinize = false;
                }
            }

            let mut state = if self_determinize {
                state.randomize()
            } else {
                state.randomize_for(state.turn())
            };

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

    pub fn simulate(&self, node_id: usize, state: &mut T) -> f32 {
        let perspective = self.tree.get_edge(node_id).unwrap().actor();

        state.do_rollout();
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
