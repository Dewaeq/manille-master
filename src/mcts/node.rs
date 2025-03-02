use super::{action_list::ActionList, edge::Edge, state::State};

pub struct Node<T: State> {
    edge: Option<Edge<T::Action, usize>>,
    parent_id: Option<usize>,
    child_ids: Vec<usize>,
    tried_actions: T::ActionList,
    is_terminal: bool,

    num_sims: usize,
    score: f32,
}

impl<T> Node<T>
where
    T: State,
{
    pub fn new(
        edge: Option<Edge<T::Action, usize>>,
        parent_id: Option<usize>,
        is_terminal: bool,
    ) -> Self {
        Node {
            edge,
            parent_id,
            tried_actions: T::ActionList::uninit(),
            child_ids: vec![],
            num_sims: 0,
            score: 0.,
            is_terminal,
        }
    }

    pub fn add_child(&mut self, child_id: usize) {
        self.child_ids.push(child_id)
    }

    pub fn has_untried_actions(&self, legal_actions: &T::ActionList) -> bool {
        !legal_actions.without(&self.tried_actions).is_empty()
    }

    pub fn pop_action(&mut self, legal_actions: &T::ActionList) -> Option<T::Action> {
        let mut actions = legal_actions.without(&self.tried_actions);
        let action = actions.pop_random();

        if let Some(action) = action.clone() {
            self.tried_actions.push(action);
        }

        action
    }

    pub fn update(&mut self, reward: f32) {
        self.num_sims += 1;
        self.score += reward;
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    pub fn edge(&self) -> Option<Edge<T::Action, usize>> {
        self.edge.clone()
    }

    pub fn child_ids(&self) -> impl Iterator<Item = &usize> {
        self.child_ids.iter()
    }

    pub const fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    pub const fn num_sims(&self) -> usize {
        self.num_sims
    }

    pub const fn avg_score(&self) -> f32 {
        self.score / self.num_sims as f32
    }

    pub fn uct_score(&self, parent_sims: usize) -> f32 {
        let n = self.num_sims as f32;
        self.score / n + 4.5 * (2. * (parent_sims as f32).ln() / n).sqrt()
    }
}
