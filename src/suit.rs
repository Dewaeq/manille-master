use std::fmt::Display;

use crate::stack::{CLUBS, DIAMONDS, HEARTS, SPADES};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Suit {
    #[default]
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    pub const fn mask(&self) -> u32 {
        match self {
            Suit::Spades => SPADES,
            Suit::Clubs => CLUBS,
            Suit::Hearts => HEARTS,
            Suit::Diamonds => DIAMONDS,
        }
    }

    pub const fn from_index(index: u32) -> Self {
        match index / 8 {
            0 => Suit::Spades,
            1 => Suit::Clubs,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => panic!(),
        }
    }
}

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(value) }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
        };

        write!(f, "{result}")
    }
}
