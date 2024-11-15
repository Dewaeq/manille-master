use super::Player;
use crate::{
    card::{Card, Cards},
    game::Game,
};

pub struct GreedyPlayer {
    cards: Cards,
    index: usize,
}

impl GreedyPlayer {
    pub fn new(index: usize) -> Self {
        GreedyPlayer {
            index,
            cards: Default::default(),
        }
    }
}

impl Player for GreedyPlayer {
    fn cards(&self) -> Cards {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Cards {
        &mut self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        let trick = &game.trick;

        // ugly, ik :(
        let mut card = {
            // if we're the first to play, play our highest card
            if trick.winner().is_none() {
                self.cards.highest().unwrap()
            }
            // otherwise, see if we can play above the current highest,
            // if not, we play our lowest card
            else if let Some(suite) = trick.suite() {
                if let Some(highest) = self.cards.highest_suite(suite) {
                    if highest.value() > trick.winner().unwrap().value() {
                        highest
                    } else {
                        self.cards.lowest().unwrap()
                    }
                } else {
                    self.cards.lowest().unwrap()
                }
            } else {
                self.cards.lowest().unwrap()
            }
        };

        card.set_player(self.index);
        card
    }
}
