use core::fmt;
use std::ops::BitXor;

use crate::bits::pop_lsb;

pub const PIJKENS: u64 = 0b1111111111111;
pub const KLAVERS: u64 = PIJKENS << 13;
pub const HARTEN: u64 = PIJKENS << 26;
pub const KOEKEN: u64 = PIJKENS << 39;

pub const ALL: u64 = PIJKENS | KLAVERS | HARTEN | KOEKEN;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suite {
    Pijkens,
    Klavers,
    Harten,
    Koeken,
}

impl Suite {
    pub fn mask(&self) -> u64 {
        match self {
            Suite::Pijkens => PIJKENS,
            Suite::Klavers => KLAVERS,
            Suite::Harten => HARTEN,
            Suite::Koeken => KOEKEN,
        }
    }
}

impl ToString for Suite {
    fn to_string(&self) -> String {
        match self {
            Suite::Harten => "♥",
            Suite::Pijkens => "♠",
            Suite::Klavers => "♣",
            Suite::Koeken => "♦",
        }
        .to_owned()
    }
}

impl From<u64> for Suite {
    fn from(value: u64) -> Self {
        unsafe { std::mem::transmute((value / 13) as u8) }
    }
}

// bit layout:
// bits 0..=3 are card value, ranging from 0..=12, with 0 being a two, 1 a three, ..., 12 an ace
// bits 4..=6 are card suite, with Pijkens = 0, Klavers = 1, Koeken = 2, Harten = 3
// bits 7..=9 are the index of the player that laid the card

#[derive(Clone, Copy)]
pub struct Card {
    data: u16,
}

impl Card {
    pub fn new(index: u64, player: usize) -> Self {
        let value = (index % 13) as u16;
        let suite = Suite::from(index);

        Self {
            data: value | ((suite as u16) << 4) | ((player as u16) << 7),
        }
    }

    pub fn to_index(&self) -> u64 {
        self.value() as u64 + (self.suite() as u64) * 13
    }

    pub fn value(&self) -> u16 {
        self.data & 0b1111
    }

    pub fn suite(&self) -> Suite {
        let suite = (self.data >> 4) & 0b111;
        unsafe { std::mem::transmute(suite as u8) }
    }

    pub fn player(&self) -> usize {
        (self.data >> 7) as usize
    }

    pub fn to_string(&self) -> String {
        let mut result = self.suite().to_string();

        let symbol = match self.value() {
            0..=8 => (self.value() + 2).to_string(),
            9 => "V".to_owned(),
            10 => "D".to_owned(),
            11 => "H".to_owned(),
            12 => "A".to_owned(),
            _ => panic!(),
        };

        result.push_str(&symbol);
        result
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Default, Clone, Copy)]
pub struct Cards {
    pub data: u64,
}

impl Cards {
    pub fn from_slice(data: &[u64]) -> Self {
        let mut cards = Cards::default();
        for i in data {
            cards.data |= 1 << i;
        }

        cards
    }

    pub fn into_iter(&self, player: usize) -> CardIterator {
        CardIterator(self.data, player)
    }
}

impl BitXor for Cards {
    type Output = Cards;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Cards {
            data: self.data ^ rhs.data,
        }
    }
}

impl BitXor<u64> for Cards {
    type Output = Cards;

    fn bitxor(self, rhs: u64) -> Self::Output {
        Cards {
            data: self.data ^ rhs,
        }
    }
}

impl BitXor<Cards> for u64 {
    type Output = Cards;

    fn bitxor(self, rhs: Cards) -> Self::Output {
        Cards {
            data: self ^ rhs.data,
        }
    }
}

//impl IntoIterator for Cards {
//type Item = Card;
//type IntoIter = CardIterator;

//fn into_iter(self) -> Self::IntoIter {
//CardIterator(self.data, 5)
//}
//}

pub struct CardIterator(u64, usize);

impl Iterator for CardIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = pop_lsb(&mut self.0);

            Some(Card::new(index, self.1))
        }
    }
}

impl fmt::Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let copy = *self;

        for card in copy.into_iter(5) {
            write!(f, "{}, ", card.to_string())?;
        }

        Ok(())
    }
}
