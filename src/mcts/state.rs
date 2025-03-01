use super::action_list::ActionList;

pub trait State {
    type Action: Clone + PartialEq;
    type ActionList: ActionList<Self::Action>;

    fn turn(&self) -> usize;

    fn randomize(&self, observer: usize) -> Self;

    fn empty_action_list(&self) -> Self::ActionList;

    fn possible_actions(&self) -> Self::ActionList;

    fn next_state(&self, action: Self::Action) -> Self
    where
        Self: Clone,
    {
        let mut next_state = self.clone();
        next_state.apply_action(action);
        next_state
    }

    fn apply_action(&mut self, action: Self::Action);

    fn is_terminal(&self) -> bool;

    fn reward(&self, perspective: usize) -> f32;
}
