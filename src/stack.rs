use core::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::{
    array::Array,
    bits::{lsb, msb, pop_lsb, pop_random_set_bit, select_random_set_bit},
    card::Card,
    suit::Suit,
};

pub const SPADES: u32 = 0b11111111;
pub const CLUBS: u32 = SPADES << 8;
pub const HEARTS: u32 = SPADES << 16;
pub const DIAMONDS: u32 = SPADES << 24;

pub const ALL: u32 = SPADES | CLUBS | HEARTS | DIAMONDS;

const TENS: u32 = 1 << 7 | 1 << 15 | 1 << 23 | 1 << 31;
const ACES: u32 = TENS >> 1;
const KINGS: u32 = TENS >> 2;
const QUEENS: u32 = TENS >> 3;
const JACKS: u32 = TENS >> 4;
const NINES: u32 = TENS >> 5;
const EIGHTS: u32 = TENS >> 6;
const SEVENS: u32 = TENS >> 7;

const HIGHEST_CARD: u32 = ACES;
const LOWEST_CARD: u32 = SEVENS;

#[derive(Default, Clone, Copy)]
pub struct Stack {
    data: u32,
}

impl Stack {
    pub const ALL: Stack = Stack { data: ALL };
    pub const BELOW: [Stack; 8] = Self::gen_below();

    const ZERO: Stack = Stack { data: 0 };

    const fn gen_below() -> [Stack; 8] {
        let mut res = [Stack::ZERO; 8];
        let mut i = 1;

        while i < 8 {
            let mut mask = 0;
            let mut j = 0;
            while j < i {
                mask |= LOWEST_CARD << j;
                j += 1;
            }

            res[i] = Stack { data: mask };
            i += 1;
        }

        res
    }

    pub fn from_slice<'a>(data: impl IntoIterator<Item = &'a u32>) -> Self {
        let mut cards = Stack::default();
        for &i in data {
            cards.data |= 1 << i;
        }

        cards
    }

    pub const fn set_data(&mut self, data: u32) {
        self.data = data;
    }

    pub const fn into_iter(self) -> CardIterator {
        CardIterator(self.data)
    }

    pub fn into_array_8(self) -> Array<Card, 8> {
        CardIterator(self.data).collect::<Array<_, 8>>()
    }

    /// useful for gettings the cards of a player,
    /// as in most games a player can have at most 13 cards
    pub fn into_array_13(self) -> Array<Card, 13> {
        CardIterator(self.data).collect::<Array<_, 13>>()
    }

    /// useful for game state stuff
    pub fn into_array_52(self) -> Array<Card, 52> {
        CardIterator(self.data).collect::<Array<_, 52>>()
    }

    pub fn pick_random_card(&self) -> Card {
        let index = select_random_set_bit(self.data);
        Card::new(index)
    }

    pub fn pop_random_card(&mut self) -> Option<Card> {
        if self.data == 0 {
            None
        } else {
            let index = pop_random_set_bit(&mut self.data);
            Some(Card::new(index))
        }
    }

    pub fn pop_lowest(&mut self) -> Option<Card> {
        if self.data == 0 {
            None
        } else {
            let index = pop_lsb(&mut self.data);
            Some(Card::new(index))
        }
    }

    pub fn push(&mut self, card: Card) {
        self.data |= 1 << card.get_index()
    }

    pub fn clear(&mut self) {
        self.data = 0;
    }

    pub fn pick_random_suit(&self) -> Suit {
        self.pick_random_card().suit()
    }

    pub const fn highest_of_suit(&self, suit: Suit) -> Option<Card> {
        let masked = self.data & suit.mask();

        if masked != 0 {
            Some(Card::from_raw(msb(masked), None, Some(suit)))
        } else {
            None
        }
    }

    pub const fn highest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (HIGHEST_CARD >> i);
            if masked != 0 {
                return Some(Card::from_raw(lsb(masked), Some(i), None));
            }
            i += 1;
        }

        None
    }

    pub const fn lowest_of_suit(&self, suit: Suit) -> Option<Card> {
        let masked = self.data & suit.mask();

        if masked != 0 {
            Some(Card::from_raw(lsb(masked), None, Some(suit)))
        } else {
            None
        }
    }

    pub const fn lowest(&self) -> Option<Card> {
        let mut i = 0;
        while i <= 12 {
            let masked = self.data & (LOWEST_CARD << i);
            if masked != 0 {
                return Some(Card::from_raw(lsb(masked), Some(i), None));
            }
            i += 1;
        }

        None
    }

    pub const fn all_below(card: Card) -> Stack {
        Self::BELOW[card.value() as usize]
    }

    pub fn all_above(card: Card) -> Stack {
        !Self::all_below(card)
    }

    pub const fn of_suit(&self, suit: Suit) -> Stack {
        Stack {
            data: self.data & suit.mask(),
        }
    }

    pub fn above(&self, card: Card) -> Stack {
        *self & Self::all_above(card)
    }

    pub fn below(&self, card: Card) -> Stack {
        *self & Self::all_below(card)
    }

    pub const fn has_suit(&self, suit: Suit) -> bool {
        self.data & suit.mask() != 0
    }

    pub const fn has_card(&self, card: Card) -> bool {
        self.has_index(card.get_index())
    }

    pub const fn has_index(&self, index: u32) -> bool {
        self.data & 1 << index != 0
    }

    pub const fn len(&self) -> u32 {
        self.data.count_ones()
    }
}

impl PartialEq for Stack {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl PartialEq<u32> for Stack {
    fn eq(&self, other: &u32) -> bool {
        self.data == *other
    }
}

impl Not for Stack {
    type Output = Self;

    fn not(self) -> Self::Output {
        Stack { data: !self.data }
    }
}

impl BitAnd for Stack {
    type Output = Stack;

    fn bitand(self, rhs: Stack) -> Self::Output {
        Stack {
            data: self.data & rhs.data,
        }
    }
}

impl BitAnd<u32> for Stack {
    type Output = Stack;

    fn bitand(self, rhs: u32) -> Self::Output {
        Stack {
            data: self.data & rhs,
        }
    }
}

impl BitAndAssign for Stack {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data &= rhs.data;
    }
}

impl BitAndAssign<u32> for Stack {
    fn bitand_assign(&mut self, rhs: u32) {
        self.data &= rhs;
    }
}

impl BitOr for Stack {
    type Output = Stack;

    fn bitor(self, rhs: Self) -> Self::Output {
        Stack {
            data: self.data | rhs.data,
        }
    }
}

impl BitOr<u32> for Stack {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        Stack {
            data: self.data | rhs,
        }
    }
}

impl BitOrAssign for Stack {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
}

impl BitXor for Stack {
    type Output = Stack;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Stack {
            data: self.data ^ rhs.data,
        }
    }
}

impl BitXor<u32> for Stack {
    type Output = Stack;

    fn bitxor(self, rhs: u32) -> Self::Output {
        Stack {
            data: self.data ^ rhs,
        }
    }
}

impl BitXorAssign<u32> for Stack {
    fn bitxor_assign(&mut self, rhs: u32) {
        self.data ^= rhs;
    }
}

impl BitOrAssign<u32> for Stack {
    fn bitor_assign(&mut self, rhs: u32) {
        self.data |= rhs;
    }
}

impl BitXor<Stack> for u32 {
    type Output = Stack;

    fn bitxor(self, rhs: Stack) -> Self::Output {
        Stack {
            data: self ^ rhs.data,
        }
    }
}

pub struct CardIterator(u32);

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

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let copy = *self;

        for card in copy.into_iter() {
            write!(f, "{}, ", card)?;
        }

        Ok(())
    }
}
