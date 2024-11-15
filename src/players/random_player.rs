use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::{Card, Cards},
    game::Game,
};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer {
    pub cards: Cards,
    index: usize,
}

impl RandomPlayer {
    pub fn new(index: usize) -> Self {
        RandomPlayer {
            index,
            cards: Cards::default(),
        }
    }
}

impl Player for RandomPlayer {
    fn cards(&self) -> Cards {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Cards {
        &mut self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        let mut rng = thread_rng();
        let my_cards = self.cards.into_iter().collect::<Vec<_>>();

        loop {
            let mut card = *my_cards.choose(&mut rng).unwrap();
            card.set_player(self.index);

            if game.is_legal(card) {
                return card;
            }
        }
    }
}
