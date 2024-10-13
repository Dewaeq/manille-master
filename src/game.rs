use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{
    card::{Card, Cards, ALL},
    //human_player::HumanPlayer,
    player::Player,
    random_player::RandomPlayer,
    trick::Trick,
};

#[derive(Default)]
pub struct Game {
    pub played_cards: Cards,
    pub trick: Trick,
    pub players: Vec<Box<dyn Player>>,
    pub dealer: usize,
    pub score: [i32; 4],
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game::default();
        for i in 0..4 {
            game.players.push(Box::new(RandomPlayer::new(i)));
        }

        game.deal_cards();
        game.next_dealer();

        game
    }

    fn next_dealer(&mut self) -> usize {
        self.dealer = (self.dealer + 1) % 4;
        self.dealer
    }

    fn deal_cards(&mut self) {
        let mut cards = (0..52).collect::<Vec<u64>>();
        cards.shuffle(&mut self.rng);

        self.players[0].set_cards(Cards::from_slice(&cards[0..13]));
        self.players[1].set_cards(Cards::from_slice(&cards[13..26]));
        self.players[2].set_cards(Cards::from_slice(&cards[26..39]));

        // Quick way to generate last set without aloop
        let last_set =
            ALL ^ self.players[0].cards() ^ self.players[1].cards() ^ self.players[2].cards();
        self.players[3].set_cards(last_set);
    }

    pub fn play_trick(&mut self) {
        self.trick = Trick::default();

        for i in self.dealer..(self.dealer + 4) {
            let idx = i % 4;
            let card = self.players[idx].decide(&self);

            self.players[idx].toggle_card(card.to_index());
            self.trick.play(card);
        }

        let winner = self.trick.winner();

        self.score[winner] += 1;
        self.dealer = winner;
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, card: Card) -> bool {
        if let Some(suite) = self.trick.suite {
            let player = &self.players[card.player()];

            player.cards().data & suite.mask() == 0 || card.suite() == suite
        } else {
            true
        }
    }
}
