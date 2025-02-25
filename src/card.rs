use crate::suite::Suite;
use std::fmt;

/// Some info on the following structures:
/// card values range from 0..=7, with 0 being a seven, 1 an eight, ..., 7 a ten
/// Format: <u8 value>: <actual card>
/// 0: seven
/// 1: eight
/// 2: nine
/// 3: jack
/// 4: queen
/// 5: king
/// 6: ace
/// 7: ten
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Card {
    value: u8,
    suite: Suite,
    index: u8,
}

impl Card {
    pub const fn new(index: u32) -> Self {
        Self::from_raw(index, None, None)
    }

    pub const fn from_raw(index: u32, value: Option<u8>, suite: Option<Suite>) -> Self {
        let value = match value {
            Some(v) => v,
            None => (index % 8) as _,
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

    pub const fn get_index(&self) -> u32 {
        self.index as _
    }

    /// Format: <u8 value>: <actual card>
    /// 0: seven
    /// 1: eight
    /// 2: nine
    /// 3: jack
    /// 4: queen
    /// 5: king
    /// 6: ace
    /// 7: ten
    pub const fn value(&self) -> u16 {
        self.value as _
    }

    pub const fn suite(&self) -> Suite {
        self.suite
    }

    pub const fn score(&self) -> i32 {
        match self.value() as i32 {
            0..=2 => 0,
            x @ 3..=7 => x - 2,
            _ => unreachable!(),
        }
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
            0..=2 => (self.value() + 7).to_string(),
            3 => "V".to_owned(),
            4 => "D".to_owned(),
            5 => "H".to_owned(),
            6 => "A".to_owned(),
            7 => "10".to_owned(),
            _ => unreachable!(),
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

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test_to_index() {
        assert!(Card::new(31).get_index() == 31);
        assert!(Card::new(4).get_index() == 4);
        assert!(Card::new(17).get_index() == 17);
    }
}
