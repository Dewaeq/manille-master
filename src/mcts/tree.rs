use super::{node::Node, state::State};

pub struct Tree<T: State> {
    nodes: Vec<Node<T>>,
    index: usize,
}

impl<T> Tree<T>
where
    T: State + Clone,
{
    pub fn add_state(&mut self, state: T, parent_id: Option<usize>) -> usize {
        let node_id = self.index;

        if let Some(parent_id) = parent_id {
            self.nodes[parent_id].add_child(node_id);
        }

        let node = Node::new(node_id, parent_id, state);

        self.nodes.push(node);
        self.index += 1;

        node_id
    }

    pub fn get_state(&self, node_id: usize) -> T {
        self.nodes[node_id].state_ref().clone()
    }

    pub fn select(&self, mut node_id: usize) -> usize {
        while self.is_fully_expanded(node_id) && !self.is_terminal(node_id) {
            node_id = self.uct_select_child(node_id).unwrap();
        }

        node_id
    }

    fn uct_select_child(&self, node_id: usize) -> Option<usize> {
        let n = self.nodes[node_id].num_sims();

        self.nodes[node_id]
            .child_ids()
            .max_by(|&&x, &&y| {
                self.nodes[x]
                    .uct_score(n)
                    .partial_cmp(&self.nodes[y].uct_score(n))
                    .unwrap()
            })
            .cloned()
    }

    pub fn expand(&mut self, node_id: usize) -> usize {
        match self.nodes[node_id].pop_action() {
            None => node_id,
            Some(action) => {
                let next_state = self.nodes[node_id].state_ref().next_state(action);
                self.add_state(next_state, Some(node_id))
            }
        }
    }

    pub fn is_fully_expanded(&self, node_id: usize) -> bool {
        !self.nodes[node_id].has_actions()
    }

    pub fn is_terminal(&self, node_id: usize) -> bool {
        self.nodes[node_id].is_terminal()
    }
}
