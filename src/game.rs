use crate::{card::Card, players::PlayerVec, stack::Stack, trick::Trick};

pub enum GameState {
    PickingTrick,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    pub played_cards: Stack,
    pub trick: Trick,
    pub players: PlayerVec,
    pub dealer: usize,
    pub turn: usize,
    pub score: [i32; 2],
}

impl Game {
    pub fn new(mut players: PlayerVec) -> Self {
        let mut game = Game::default();

        for (i, player) in players.iter_mut().enumerate() {
            player.set_index(i);
        }

        game.players = players;
        game.pick_random_dealer();
        game.deal_cards(Stack::ALL);

        game
    }

    pub fn pick_random_dealer(&mut self) {
        self.dealer = romu::mod_usize(4);
        self.turn = (self.dealer + 1) % 4;
    }

    pub fn next_dealer(&mut self) {
        self.dealer = (self.dealer + 1) % 4;
    }

    /// evenly divide the given stack over all players
    pub fn deal_cards(&mut self, stack: Stack) {
        let mut indices = (0..32).filter(|&x| stack.has_index(x)).collect::<Vec<_>>();
        let mut cards = [0; 3];

        // number of cards per player
        let n = indices.len() / 4;

        for i in (0..3 * n).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[i / n] |= 1 << indices[i];
        }

        self.players[0].cards_mut().set_data(cards[0]);
        self.players[1].cards_mut().set_data(cards[1]);
        self.players[2].cards_mut().set_data(cards[2]);
        self.players[3].set_cards(stack ^ cards[0] ^ cards[1] ^ cards[2]);
    }

    /// returns the winning team and the score of all cards in this trick
    pub fn play_trick(&mut self) -> (usize, i32) {
        self.trick.clear();

        for i in self.turn..(self.turn + 4) {
            let player_idx = i % 4;
            let card = self.players[player_idx].decide(self);
            let card_idx = card.get_index();

            assert!(self.is_legal(card, player_idx));

            self.players[player_idx].toggle_card(card_idx);
            self.played_cards |= 1 << card_idx;
            self.trick.play(card, player_idx);
        }

        let winner = self.trick.winning_player().unwrap();
        // the winner of a trick get's to play first in the next trick
        self.turn = winner;
        let winning_team = winner % 2;

        (winning_team, self.trick.score())
    }

    /// play an entire round, i.e. 8 tricks
    /// this method also assigns the next dealer
    pub fn play_round(&mut self) {
        let trump = self.players[self.dealer].pick_trump(self);
        self.trick.set_trump(Some(trump));

        let mut scores = [0; 2];

        for _ in 0..8 {
            let (winning_team, score) = self.play_trick();
            scores[winning_team] += score;
        }

        assert!(scores.iter().sum::<i32>() == 60);

        let (winning_team, &score) = scores.iter().enumerate().max_by_key(|(_, &y)| y).unwrap();
        self.score[winning_team] += 30 - score;

        self.next_dealer();
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, card: Card, player: usize) -> bool {
        if let Some(suite) = self.trick.suite_to_follow() {
            let player = &self.players[player];

            !player.cards().has_suite(suite) || card.suite() == suite
        } else {
            true
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.score.iter().any(|&s| s >= 101)
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::{
        players::{random_player::RandomPlayer, Player, PlayerVec},
        stack::Stack,
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
        let mut seen_cards = Stack::default();

        for player in &game.players {
            let cards = player.cards();
            seen_cards |= cards;

            assert!(cards.len() == 32 / (game.players.len() as u32));
        }

        assert!(seen_cards == Stack::ALL);
    }

    #[test]
    fn test_random_game() {
        let players: PlayerVec = vec![
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
            RandomPlayer::boxed(),
        ];

        let mut game = Game::new(players);
        game.play_round();
    }
}
