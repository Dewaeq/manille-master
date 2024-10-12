use crate::card::{Card, Suite};

#[derive(Default)]
pub struct Trick {
    cards: Vec<Card>,
    pub suite: Option<Suite>,
}

impl Trick {
    pub fn play(&mut self, card: Card) {
        if self.suite.is_none() {
            self.suite = Some(card.suite());
        }

        self.cards.push(card);
    }

    pub fn winner(&self) -> usize {
        let trick_suite = self.suite.unwrap();
        let mut winner = self.cards[0];

        for card in &self.cards[1..] {
            let suite = card.suite();
            let value = card.value();

            if suite == trick_suite && winner.suite() != trick_suite
                || value > winner.value() && (suite == winner.suite())
            {
                winner = *card;
            }
        }

        winner.player()
    }
}
