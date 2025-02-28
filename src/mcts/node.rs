use crate::action_list::ActionList;

use super::state::State;

pub struct Node<T: State> {
    id: usize,
    state: T,
    parent_id: Option<usize>,
    child_ids: Vec<usize>,
    actions: T::ActionList,

    num_sims: usize,
    score: f32,
}

impl<T> Node<T>
where
    T: State,
{
    pub fn new(id: usize, parent_id: Option<usize>, state: T) -> Self {
        let actions = state.possible_actions();

        Node {
            id,
            state,
            parent_id,
            actions,
            child_ids: vec![],
            num_sims: 0,
            score: 0.,
        }
    }

    pub fn add_child(&mut self, child_id: usize) {
        self.child_ids.push(child_id)
    }

    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    pub fn pop_action(&mut self) -> Option<T::Action> {
        self.actions.pop()
    }

    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    pub fn child_ids(&self) -> impl Iterator<Item = &usize> {
        self.child_ids.iter()
    }

    pub const fn state_ref(&self) -> &T {
        &self.state
    }

    pub const fn num_sims(&self) -> usize {
        self.num_sims
    }

    pub fn uct_score(&self, parent_sims: usize) -> f32 {
        let n = self.num_sims as f32;
        self.score / n + (2. * (parent_sims as f32).ln() / n).sqrt()
    }
}
