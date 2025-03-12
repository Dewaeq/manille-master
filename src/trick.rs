use std::fmt::Debug;

use crate::{array::Array, card::Card, suit::Suit};

#[derive(Default, Clone, Copy)]
pub struct Trick {
    /// TODO: might be able to remove this field
    cards: Array<Card, 4>,
    trump: Option<Suit>,
    winner: Option<(Card, usize)>,
    score: i16,
}

impl Trick {
    /// clear all properties except trump
    pub const fn clear(&mut self) {
        self.cards.clear();
        self.winner = None;
        self.score = 0;
    }

    pub const fn set_trump(&mut self, trump: Option<Suit>) {
        self.trump = trump;
    }

    pub fn play(&mut self, card: Card, player: usize) {
        match self.winner {
            // this is the first card of this trick
            None => self.winner = Some((card, player)),
            Some((winner_card, _)) => match self.trump {
                // playing with trump
                Some(trump) => {
                    let winner_suit = winner_card.suit();
                    let card_suit = card.suit();

                    if (card_suit == trump && winner_suit != trump)
                        || (card.value() > winner_card.value() && card_suit == winner_suit)
                    {
                        self.winner = Some((card, player));
                    }
                }
                // playing without trump
                None => {
                    if card.suit() == winner_card.suit() && card.value() > winner_card.value() {
                        self.winner = Some((card, player));
                    }
                }
            },
        }

        self.score += card.score();
        self.cards.push(card);
    }

    pub const fn winner(&self) -> Option<(Card, usize)> {
        self.winner
    }

    pub const fn score(&self) -> i16 {
        self.score
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.cards.get(0).map(|c| c.suit())
    }

    pub const fn trump(&self) -> Option<Suit> {
        self.trump
    }

    pub const fn is_finished(&self) -> bool {
        self.cards.len() == 4
    }

    pub const fn cards(&self) -> Array<Card, 4> {
        self.cards
    }
}

impl Debug for Trick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trick")
            .field("trump", &self.trump)
            .field("winner", &self.winner)
            .field("score", &self.score)
            .finish()
    }
}
