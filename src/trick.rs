use crate::{
    array::Array,
    card::{Card, Suite},
};

#[derive(Default)]
pub struct Trick {
    cards: Array<Card, 4>,
    suite: Option<Suite>,
    winner: Option<Card>,
}

impl Trick {
    pub fn clear(&mut self) {
        self.cards.clear();
        self.suite = None;
        self.winner = None;
    }

    pub fn play(&mut self, card: Card) {
        if self.suite.is_none() {
            self.suite = Some(card.suite());
            self.winner = Some(card);
        } else {
            let trick_suite = self.suite.unwrap();
            let suite = card.suite();
            let winner = self.winner.unwrap();

            if suite == trick_suite && winner.suite() != trick_suite
                || card.value() > winner.value() && (suite == winner.suite())
            {
                self.winner = Some(card);
            }
        }

        self.cards.push(card);
    }

    pub fn winner(&self) -> Option<Card> {
        self.winner
    }

    pub fn suite(&self) -> Option<Suite> {
        self.suite
    }
}
