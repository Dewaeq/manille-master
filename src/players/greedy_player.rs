use super::Player;
use crate::{action::Action, game_state::GameState, stack::Stack, suite::Suite};

#[derive(Default)]
pub struct GreedyPlayer {
    cards: Stack,
    index: usize,
}

impl Player for GreedyPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn cards(&self) -> Stack {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Stack {
        &mut self.cards
    }

    fn decide(&mut self, state: GameState) -> Action {
        todo!()

        //let trick = &game.trick;
        //
        //match trick.winning_card() {
        //    // if we're the first to play, play our highest card
        //    None => return self.cards.highest().unwrap(),
        //    // otherwise, see if we can play above the current highest,
        //    // if not, we play our lowest card, while following the suite
        //    // if possible
        //    Some(winning_card) => {
        //        // a card has already been played, so we're sure that
        //        // suite has been initialised
        //        let suite = trick.suite_to_follow().unwrap();
        //
        //        if self.cards.has_suite(suite) {
        //            if let Some(highest) = self.cards.highest_of_suite(suite) {
        //                if highest.value() > winning_card.value() {
        //                    return highest;
        //                }
        //            }
        //
        //            return self.cards.lowest_of_suite(suite).unwrap();
        //        }
        //    }
        //}
        //
        //self.cards.lowest().unwrap()
    }

    //fn pick_trump(&self, _state: GameState) -> Option<Suite> {
    //    todo!()
    //}
}
