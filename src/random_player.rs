use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::{Card, Cards},
    game::Game,
    player::Player,
};

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
    fn set_cards(&mut self, cards: Cards) {
        self.cards = cards;
    }

    fn toggle_card(&mut self, index: u64) {
        self.cards.data ^= 1 << index;
    }

    fn cards(&self) -> Cards {
        self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        let mut rng = thread_rng();
        let my_cards = self.cards.into_iter(self.index).collect::<Vec<_>>();

        loop {
            let card = *my_cards.choose(&mut rng).unwrap();

            if game.is_legal(card) {
                return card;
            }
        }
    }
}
