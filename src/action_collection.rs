use std::fmt::Debug;

use ismcts::action_list::ActionList;

use crate::{
    action::Action,
    bits::{pop_lsb, select_random_set_bit},
    stack::Stack,
    suit::Suit,
};

const NO_TRUMP_INDEX: u8 = 4;
const NO_TRUMP_MASK: u8 = 1 << NO_TRUMP_INDEX;

#[derive(Clone, Copy)]
pub enum ActionCollection {
    Cards(Stack),
    /// bit 0..=3 are your regular suits
    /// bit 4 means without trump
    Trumps(u8),
    Uninit,
}

impl ActionCollection {
    pub fn to_vec(self) -> Vec<Action> {
        match self {
            ActionCollection::Cards(stack) => stack.into_iter().map(Action::PlayCard).collect(),
            ActionCollection::Trumps(bits) => {
                let mut actions = vec![];
                let mut bits = bits as u32;
                while bits != 0 {
                    let idx = pop_lsb(&mut bits);
                    let action = if idx == NO_TRUMP_INDEX as _ {
                        None
                    } else {
                        Some(Suit::from(idx as u8))
                    };

                    actions.push(Action::PickTrump(action));
                }

                actions
            }
            ActionCollection::Uninit => vec![],
        }
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
                if let Some(suit) = trump {
                    *bits |= 1 << suit as u8;
                } else {
                    *bits |= 1 << NO_TRUMP_INDEX;
                }
            }
            (this @ ActionCollection::Uninit, Action::PlayCard(card)) => {
                let stack = Stack::default() | 1 << card.get_index();
                *this = ActionCollection::Cards(stack);
            }
            (this @ ActionCollection::Uninit, Action::PickTrump(trump)) => {
                let bits = if let Some(suit) = trump {
                    1 << suit as u8
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
                        Some(Suit::from(index as u8))
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
                if let Some(suit) = trump {
                    *bits & 1 << *suit as u8 != 0
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

impl Debug for ActionCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cards(stack) => writeln!(f, "{stack:?}"),
            Self::Trumps(bits) => {
                let mut suits = [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds]
                    .into_iter()
                    .filter(|&s| bits & 1 << s as u8 != 0)
                    .map(Some)
                    .collect::<Vec<_>>();
                if bits & NO_TRUMP_MASK != 0 {
                    suits.push(None);
                }

                writeln!(f, "{suits:?}")
            }
            Self::Uninit => writeln!(f, "Uninit"),
        }
    }
}
