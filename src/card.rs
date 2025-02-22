use core::fmt;
use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

use crate::{
    array::Array,
    bits::{lsb, msb, pop_lsb, select_random_set_bit},
};

pub const PIJKENS: u64 = 0b1111111111111;
pub const KLAVERS: u64 = PIJKENS << 13;
pub const HARTEN: u64 = PIJKENS << 26;
pub const KOEKEN: u64 = PIJKENS << 39;

pub const ALL: u64 = PIJKENS | KLAVERS | HARTEN | KOEKEN;

pub const ACES: u64 = 1 << 12 | 1 << 25 | 1 << 38 | 1 << 51;
pub const TWOS: u64 = ACES >> 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Suite {
    #[default]
    Pijkens,
    Klavers,
    Harten,
    Koeken,
}

impl Suite {
    pub const fn mask(&self) -> u64 {
        match self {
            Suite::Pijkens => PIJKENS,
            Suite::Klavers => KLAVERS,
            Suite::Harten => HARTEN,
            Suite::Koeken => KOEKEN,
        }
    }

    pub const fn from_index(index: u64) -> Self {
        match index / 13 {
            0 => Suite::Pijkens,
            1 => Suite::Klavers,
            2 => Suite::Harten,
            3 => Suite::Koeken,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Card {
    value: u8,
    suite: Suite,
    index: u8,
}

impl Card {
    pub const fn new(index: u64) -> Self {
        Self::from_raw(index, None, None)
    }

    pub const fn from_raw(index: u64, value: Option<u8>, suite: Option<Suite>) -> Self {
        let value = match value {
            Some(v) => v,
            None => (index % 13) as _,
        };
        let suite = match suite {
            Some(s) => s,
            None => Suite::from_index(index),
        };

        Self {
            value,
            index: index as _,
            suite,
        }
    }

    pub const fn get_index(&self) -> u64 {
        self.index as _
    }

    pub const fn value(&self) -> u16 {
        self.value as _
    }

    pub const fn suite(&self) -> Suite {
        self.suite
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = match self.suite() {
            Suite::Harten => "♥",
            Suite::Pijkens => "♠",
            Suite::Klavers => "♣",
            Suite::Koeken => "♦",
        }
        .to_owned();

        let symbol = match self.value() {
            0..=8 => (self.value() + 2).to_string(),
            9 => "V".to_owned(),
            10 => "D".to_owned(),
            11 => "H".to_owned(),
            12 => "A".to_owned(),
            _ => panic!(),
        };

        result.push_str(&symbol);

        write!(f, "{}", result)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Cards {
    data: u64,
}

impl Cards {
    pub fn from_slice(data: &[u64]) -> Self {
        let mut cards = Cards::default();
        for i in data {
            cards.data |= 1 << i;
        }

        cards
    }

    pub const fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    pub const fn into_iter(self) -> CardIterator {
        CardIterator(self.data)
    }

    pub fn into_array_13(self) -> Array<Card, 13> {
        CardIterator(self.data).collect::<Array<_, 13>>()
    }

    pub fn into_array_52(self) -> Array<Card, 52> {
        CardIterator(self.data).collect::<Array<_, 52>>()
    }

    pub fn pick_random_card(&self) -> Card {
        let index = select_random_set_bit(self.data);
        Card::new(index)
    }

    pub const fn highest_of_suite(&self, suite: Suite) -> Option<Card> {
        let masked = self.data & suite.mask();

        if masked != 0 {
            Some(Card::from_raw(msb(masked), None, Some(suite)))
        } else {
            None
        }
    }

    pub const fn highest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (ACES >> i);
            if masked != 0 {
                return Some(Card::from_raw(lsb(masked), Some(i), None));
            }
            i += 1;
        }

        None
    }

    pub const fn lowest_of_suite(&self, suite: Suite) -> Option<Card> {
        let masked = self.data & suite.mask();

        if masked != 0 {
            Some(Card::from_raw(lsb(masked), None, Some(suite)))
        } else {
            None
        }
    }

    pub const fn lowest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (TWOS << i);
            if masked != 0 {
                return Some(Card::from_raw(lsb(masked), Some(i), None));
            }
            i += 1;
        }

        None
    }

    pub const fn has_suite(&self, suite: Suite) -> bool {
        self.data & suite.mask() != 0
    }

    pub const fn len(&self) -> u32 {
        self.data.count_ones()
    }
}

impl PartialEq<u64> for Cards {
    fn eq(&self, other: &u64) -> bool {
        self.data == *other
    }
}

impl BitAnd<u64> for Cards {
    type Output = Cards;

    fn bitand(self, rhs: u64) -> Self::Output {
        Cards {
            data: self.data & rhs,
        }
    }
}

impl BitOr for Cards {
    type Output = Cards;

    fn bitor(self, rhs: Self) -> Self::Output {
        Cards {
            data: self.data | rhs.data,
        }
    }
}
impl BitOrAssign for Cards {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
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

impl BitXorAssign<u64> for Cards {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.data ^= rhs;
    }
}

impl BitOrAssign<u64> for Cards {
    fn bitor_assign(&mut self, rhs: u64) {
        self.data |= rhs;
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

pub struct CardIterator(u64);

impl Iterator for CardIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = pop_lsb(&mut self.0);

            Some(Card::new(index))
        }
    }
}

impl fmt::Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let copy = *self;

        for card in copy.into_iter() {
            write!(f, "{}, ", card)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test_to_index() {
        assert!(Card::new(34).get_index() == 34);
        assert!(Card::new(4).get_index() == 4);
        assert!(Card::new(17).get_index() == 17);
    }
}
