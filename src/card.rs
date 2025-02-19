use core::fmt;
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::bits::{lsb, msb, pop_lsb};

pub const PIJKENS: u64 = 0b1111111111111;
pub const KLAVERS: u64 = PIJKENS << 13;
pub const HARTEN: u64 = PIJKENS << 26;
pub const KOEKEN: u64 = PIJKENS << 39;

pub const ALL: u64 = PIJKENS | KLAVERS | HARTEN | KOEKEN;

pub const ACES: u64 = 1 << 12 | 1 << 25 | 1 << 38 | 1 << 51;
pub const TWOS: u64 = ACES >> 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suite {
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

// bit layout:
// bits 0..=3 are card value, ranging from 0..=12, with 0 being a two, 1 a three, ..., 12 an ace
// bits 4..=6 are card suite, with Pijkens = 0, Klavers = 1, Koeken = 2, Harten = 3
// bits 7..=9 are the index of the player that laid the card,
//            value 5 means the player is unkown

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Card {
    data: u16,
}

impl Card {
    pub const fn new(index: u64) -> Self {
        let value = (index % 13) as u16;
        let suite = Suite::from_index(index);

        Self {
            data: value | ((suite as u16) << 4) | (5u16 << 7),
        }
    }

    pub const fn get_index(&self) -> u64 {
        self.value() as u64 + (self.suite() as u64) * 13
    }

    pub const fn value(&self) -> u16 {
        self.data & 0b1111
    }

    pub const fn suite(&self) -> Suite {
        let suite = (self.data >> 4) & 0b111;
        unsafe { std::mem::transmute(suite as u8) }
    }

    pub const fn set_player(&mut self, player: usize) {
        self.data &= 0b1111111;
        self.data |= (player as u16) << 7;
    }

    pub const fn player(&self) -> usize {
        (self.data >> 7) as usize
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
        write!(f, "{}, player: {}", self, self.player())
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

    pub const fn into_iter(self) -> CardIterator {
        CardIterator(self.data)
    }

    pub const fn highest_of_suite(&self, suite: Suite) -> Option<Card> {
        let masked = self.data & suite.mask();

        if masked != 0 {
            Some(Card::new(msb(masked)))
        } else {
            None
        }
    }

    pub const fn highest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (ACES >> i);
            if masked != 0 {
                return Some(Card::new(lsb(masked)));
            }
            i += 1;
        }

        None
    }

    pub const fn lowest_of_suite(&self, suite: Suite) -> Option<Card> {
        let masked = self.data & suite.mask();

        if masked != 0 {
            Some(Card::new(lsb(masked)))
        } else {
            None
        }
    }

    pub const fn lowest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (TWOS << i);
            if masked != 0 {
                return Some(Card::new(lsb(masked)));
            }
            i += 1;
        }

        None
    }

    pub const fn has(&self, suite: Suite) -> bool {
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

//impl IntoIterator for Cards {
//type Item = Card;
//type IntoIter = CardIterator;

//fn into_iter(self) -> Self::IntoIter {
//CardIterator(self.data, 5)
//}
//}

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

        let mut card = Card::new(26);
        card.set_player(2);
        assert!(card.player() == 2);
    }
}
