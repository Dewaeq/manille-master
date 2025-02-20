use crate::{
    card::{Card, Cards, ALL},
    players::PlayerVec,
    trick::Trick,
};

#[derive(Default)]
pub struct Game {
    pub played_cards: Cards,
    pub trick: Trick,
    pub players: PlayerVec,
    pub dealer: usize,
    pub score: [i32; 4],
}

impl Game {
    pub fn new(mut players: PlayerVec) -> Self {
        let mut game = Game::default();

        for (i, player) in players.iter_mut().enumerate() {
            player.set_index(i);
        }

        //game.dealer = game.rng.gen_range(0..=3);
        game.dealer = romu::mod_usize(4);
        game.players = players;
        game.deal_cards();

        game
    }

    pub fn next_dealer(&mut self) -> usize {
        self.dealer = (self.dealer + 1) % 4;
        self.dealer
    }

    pub fn deal_cards(&mut self) {
        let mut indices: [u64; 52] = std::array::from_fn(|i| i as u64);

        let mut cards = [0; 3];
        for i in (0..39).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[i / 13] |= 1 << indices[i];
        }

        self.players[0].cards_mut().set_data(cards[0]);
        self.players[1].cards_mut().set_data(cards[1]);
        self.players[2].cards_mut().set_data(cards[2]);
        self.players[3]
            .cards_mut()
            .set_data(ALL ^ cards[0] ^ cards[1] ^ cards[2]);
    }

    pub fn play_trick(&mut self) {
        self.trick.clear();

        for i in self.dealer..(self.dealer + 4) {
            let player_idx = i % 4;
            let card = self.players[player_idx].decide(self);
            let card_idx = card.get_index();

            self.players[player_idx].toggle_card(card_idx);
            self.played_cards |= 1 << card_idx;
            self.trick.play(card);
        }

        let winner = self.trick.winner().unwrap();

        self.score[winner.player()] += 1;
        self.dealer = winner.player();
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, card: Card) -> bool {
        if let Some(suite) = self.trick.suite() {
            let player = &self.players[card.player()];

            player.cards() & suite.mask() == 0 || card.suite() == suite
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::{
        card::{Cards, ALL},
        players::{random_player::RandomPlayer, Player, PlayerVec},
    };

    #[test]
    fn test_dealing() {
        let players: PlayerVec = vec![
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
        ];

        let game = Game::new(players);
        let mut all_cards = Cards::default();

        for player in &game.players {
            let cards = player.cards();
            all_cards |= cards;

            assert!(cards.len() == 52 / (game.players.len() as u32));
        }

        assert!(all_cards == ALL);
    }
}
