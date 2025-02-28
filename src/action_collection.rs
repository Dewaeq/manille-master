use crate::{
    action::Action, bits::select_random_set_bit, mcts::action_list::ActionList, stack::Stack,
    suite::Suite,
};

#[derive(Clone, Copy)]
pub enum ActionCollection {
    Cards(Stack),
    /// bit 0..=3 are your regular suites
    /// bit 4 means without trump
    Trumps(u8),
}

impl ActionList<Action> for ActionCollection {
    fn push(&mut self, action: Action) {
        match (self, action) {
            (ActionCollection::Cards(stack), Action::PlayCard(card)) => {
                *stack |= 1 << card.get_index()
            }
            (ActionCollection::Trumps(bits), Action::PickTrump(trump)) => {
                if let Some(suite) = trump {
                    *bits |= 1 << suite as u8;
                } else {
                    *bits |= 1 << 4;
                }
            }
            _ => unreachable!(),
        }
    }

    fn pop_random(&mut self) -> Option<Action> {
        match self {
            ActionCollection::Cards(stack) => stack.pop_random_card().map(Action::PlayCard),
            ActionCollection::Trumps(bits) => {
                if *bits == 0 {
                    None
                } else {
                    let index = select_random_set_bit(*bits as _);
                    let choice = if index == 4 {
                        None
                    } else {
                        Some(unsafe { std::mem::transmute::<u8, Suite>(index as u8) })
                    };

                    *bits ^= 1 << index;

                    Some(Action::PickTrump(choice))
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            ActionCollection::Cards(stack) => stack.len() == 0,
            ActionCollection::Trumps(bits) => *bits == 0,
        }
    }

    fn has(&self, item: &Action) -> bool {
        match (self, item) {
            (ActionCollection::Cards(stack), Action::PlayCard(card)) => stack.has_card(*card),
            (ActionCollection::Trumps(bits), Action::PickTrump(trump)) => {
                if let Some(suite) = trump {
                    *bits & 1 << *suite as u8 != 0
                } else {
                    *bits & 1 << 4 != 0
                }
            }
            _ => unreachable!(),
        }
    }

    fn without(&self, other: &Self) -> Self {
        match (self, other) {
            (ActionCollection::Cards(stack), ActionCollection::Cards(other_stack)) => {
                ActionCollection::Cards(*stack & !*other_stack)
            }
            (ActionCollection::Trumps(bits), ActionCollection::Trumps(other_bits)) => {
                ActionCollection::Trumps(*bits & !*other_bits)
            }
            _ => unreachable!(),
        }
    }
}
