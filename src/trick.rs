use crate::{
    array::Array,
    card::{Card, Suite},
};

#[derive(Default)]
pub struct Trick {
    cards: Array<Card, 4>,
    suite: Option<Suite>,
    winner: Option<(Card, usize)>,
}

impl Trick {
    pub fn clear(&mut self) {
        self.cards.clear();
        self.suite = None;
        self.winner = None;
    }

    pub fn play(&mut self, card: Card, player: usize) {
        if self.suite.is_none() {
            self.suite = Some(card.suite());
            self.winner = Some((card, player));
        } else {
            let trick_suite = self.suite.unwrap();
            let suite = card.suite();
            let (winner_card, _) = self.winner.unwrap();

            if suite == trick_suite && winner_card.suite() != trick_suite
                || card.value() > winner_card.value() && (suite == winner_card.suite())
            {
                self.winner = Some((card, player));
            }
        }

        self.cards.push(card);
    }

    pub fn winning_card(&self) -> Option<Card> {
        self.winner.map(|(card, _)| card)
    }

    pub fn winning_player(&self) -> Option<usize> {
        self.winner.map(|(_, player)| player)
    }

    pub const fn suite(&self) -> Option<Suite> {
        self.suite
    }
}
