use std::fmt::Display;

use crate::stack::{HARTEN, KLAVERS, KOEKEN, PIJKENS};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Suite {
    #[default]
    Pijkens,
    Klavers,
    Harten,
    Koeken,
}

impl Suite {
    pub const fn mask(&self) -> u32 {
        match self {
            Suite::Pijkens => PIJKENS,
            Suite::Klavers => KLAVERS,
            Suite::Harten => HARTEN,
            Suite::Koeken => KOEKEN,
        }
    }

    pub const fn from_index(index: u32) -> Self {
        match index / 8 {
            0 => Suite::Pijkens,
            1 => Suite::Klavers,
            2 => Suite::Harten,
            3 => Suite::Koeken,
            _ => panic!(),
        }
    }
}

impl From<u8> for Suite {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(value) }
    }
}

impl Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Suite::Harten => "♥",
            Suite::Pijkens => "♠",
            Suite::Klavers => "♣",
            Suite::Koeken => "♦",
        };

        write!(f, "{result}")
    }
}
