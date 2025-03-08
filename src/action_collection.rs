use std::fmt::{Debug, Display};

use crate::{
    action::Action,
    bits::{lsb, pop_lsb, select_random_set_bit},
    mcts::action_list::ActionList,
    stack::Stack,
    suite::Suite,
};

const NO_TRUMP_INDEX: u8 = 4;
const NO_TRUMP_MASK: u8 = 1 << NO_TRUMP_INDEX;

#[derive(Clone, Copy)]
pub enum ActionCollection {
    Cards(Stack),
    /// bit 0..=3 are your regular suites
    /// bit 4 means without trump
    Trumps(u8),
    Uninit,
}

impl ActionCollection {
    pub fn to_vec(self) -> Vec<Action> {
        let mut results = vec![];

        match self {
            ActionCollection::Cards(stack) => {
                for card in stack.into_iter() {
                    results.push(Action::PlayCard(card));
                }
            }
            ActionCollection::Trumps(mut bits) => {
                while bits != 0 {
                    let index = lsb(bits as _);
                    let choice = if index == NO_TRUMP_INDEX as _ {
                        None
                    } else {
                        Some(unsafe { std::mem::transmute::<u8, Suite>(index as u8) })
                    };

                    results.push(Action::PickTrump(choice));
                    bits ^= 1 << index;
                }
            }
            ActionCollection::Uninit => (),
        }

        results
    }
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
                    *bits |= 1 << NO_TRUMP_INDEX;
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
                    1 << NO_TRUMP_INDEX
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
                    let choice = if index == NO_TRUMP_INDEX as _ {
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
                    *bits & NO_TRUMP_MASK != 0
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

impl Display for ActionCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cards(stack) => {
                for (i, card) in stack.into_iter().enumerate() {
                    writeln!(f, "{i}: {card}")?;
                }
                Ok(())
            }
            Self::Trumps(bits) => {
                let mut trumps = [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken]
                    .into_iter()
                    .filter(|&s| bits & 1 << s as u8 != 0)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                if bits & NO_TRUMP_MASK != 0 {
                    trumps.push("no trump".to_owned());
                }

                for (i, trump) in trumps.iter().enumerate() {
                    writeln!(f, "{i}: {trump}")?;
                }

                Ok(())
            }
            Self::Uninit => writeln!(f, "Uninit"),
        }
    }
}

impl Debug for ActionCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cards(stack) => writeln!(f, "{stack:?}"),
            Self::Trumps(bits) => {
                let mut suites = [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken]
                    .into_iter()
                    .filter(|&s| bits & 1 << s as u8 != 0)
                    .map(Some)
                    .collect::<Vec<_>>();
                if bits & NO_TRUMP_MASK != 0 {
                    suites.push(None);
                }

                writeln!(f, "{suites:?}")
            }
            Self::Uninit => writeln!(f, "Uninit"),
        }
    }
}
