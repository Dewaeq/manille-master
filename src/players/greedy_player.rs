use super::Player;
use crate::{
    card::{Card, Cards},
    game::Game,
};

#[derive(Default)]
pub struct GreedyPlayer {
    cards: Cards,
    index: usize,
}

impl Player for GreedyPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn cards(&self) -> Cards {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Cards {
        &mut self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        let trick = &game.trick;

        match trick.winning_card() {
            // if we're the first to play, play our highest card
            None => return self.cards.highest().unwrap(),
            // otherwise, see if we can play above the current highest,
            // if not, we play our lowest card, while following the suite
            // if possible
            Some(winning_card) => {
                // a card has already been played, so we're sure that
                // suite has been initialised
                let suite = trick.suite().unwrap();

                if self.cards & suite.mask() != 0 {
                    if let Some(highest) = self.cards.highest_of_suite(suite) {
                        if highest.value() > winning_card.value() {
                            return highest;
                        }
                    }

                    return self.cards.lowest_of_suite(suite).unwrap();
                }
            }
        }

        self.cards.lowest().unwrap()
    }
}
