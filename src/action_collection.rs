use std::fmt::Debug;

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
    Uninit,
}

impl ActionList<Action> for ActionCollection {
    fn uninit() -> Self {
        ActionCollection::Uninit
    }

    fn push(&mut self, action: Action) {
        match (self, action) {
            (ActionCollection::Cards(ref mut stack), Action::PlayCard(card)) => {
                *stack |= 1 << card.get_index()
            }
            (ActionCollection::Trumps(bits), Action::PickTrump(trump)) => {
                if let Some(suite) = trump {
                    *bits |= 1 << suite as u8;
                } else {
                    *bits |= 1 << 4;
                }
            }
            (this @ ActionCollection::Uninit, Action::PlayCard(card)) => {
                let stack = Stack::default() | 1 << card.get_index();
                *this = ActionCollection::Cards(stack);
            }
            (this @ ActionCollection::Uninit, Action::PickTrump(trump)) => {
                let bits = if let Some(suite) = trump {
                    1 << suite as u8
                } else {
                    1 << 4
                };

                *this = ActionCollection::Trumps(bits);
            }
            _ => unreachable!(),
        }
    }

    fn pop_random(&mut self) -> Option<Action> {
        match self {
            ActionCollection::Uninit => unreachable!(),
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

    fn len(&self) -> usize {
        match self {
            ActionCollection::Cards(stack) => stack.len() as _,
            ActionCollection::Trumps(bits) => bits.count_ones() as _,
            ActionCollection::Uninit => 0,
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
            (ActionCollection::Uninit, _) => false,
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
            (this, ActionCollection::Uninit) => *this,
            _ => unreachable!(),
        }
    }
}

impl Debug for ActionCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cards(stack) => writeln!(f, "{stack:?}"),
            Self::Trumps(bits) => {
                let suites = [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken]
                    .into_iter()
                    .filter(|&s| bits & 1 << s as u8 != 0)
                    .collect::<Vec<_>>();

                writeln!(f, "{suites:?}")
            }
            Self::Uninit => writeln!(f, "Uninit"),
        }
    }
}
