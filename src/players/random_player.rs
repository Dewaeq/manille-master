use crate::{card::Card, game::Game, stack::Stack, suite::Suite};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer {
    cards: Stack,
    index: usize,
}

impl Player for RandomPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn cards(&self) -> Stack {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Stack {
        &mut self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        let mut cards = self.cards;

        // have to follow if possible,
        // if we can't follow, we must buy
        if let Some(suite) = game
            .trick
            .suite_to_follow()
            .filter(|&s| self.cards.has_suite(s))
        {
            let filtered_cards = cards & suite.mask();
            if filtered_cards != 0 {
                cards = filtered_cards;
            }
        }

        // this also means we're not the first player, i.e. the suite
        // to follow has been determined
        if let Some((winning_card, winning_player)) = game.trick.winner() {
            // our team is winning
            if winning_player % 2 == self.index % 2 {
                todo!();
            } else {
                if true {
                    todo!()
                }

                // can't 'under-buy', except if that's our only possible move
                if let Some(trump) = game.trick.trump().filter(|&t| winning_card.suite() == t) {
                    let mask = Stack::BELOW[winning_card.value() as usize] & trump.mask();
                    let filtered_cards = cards & !mask;

                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
            }
        }

        loop {
            let card = cards.pick_random_card();

            if game.is_legal(card, self.index) {
                return card;
            }
        }
    }

    fn pick_trump(&self, _game: &Game) -> Suite {
        self.cards.pick_random_suite()
    }
}
