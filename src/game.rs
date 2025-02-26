use std::fmt::Debug;

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
        game.set_random_dealer();
        game.deal_cards();

        game
    }

    pub fn set_random_dealer(&mut self) {
        self.dealer = romu::mod_usize(4);
        self.turn = (self.dealer + 1) % 4;
    }

    pub fn set_next_dealer(&mut self) {
        self.dealer = (self.dealer + 1) % 4;
        self.turn = (self.dealer + 1) % 4;
    }

    /// evenly divide the given stack over all players
    pub fn deal_cards(&mut self) {
        //let mut indices = (0..32).filter(|&x| stack.has_index(x)).collect::<Vec<_>>();
        let mut indices: [u32; 32] = std::array::from_fn(|i| i as u32);
        let mut cards = [0; 3];

        // number of cards per player
        let n = indices.len() / 4;

        for i in (n..4 * n).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[(i / n) - 1] |= 1 << indices[i];
        }

        self.players[0].cards_mut().set_data(cards[0]);
        self.players[1].cards_mut().set_data(cards[1]);
        self.players[2].cards_mut().set_data(cards[2]);
        self.players[3].set_cards(Stack::ALL ^ cards[0] ^ cards[1] ^ cards[2]);
    }

    /// returns the winning team and the score of all cards in this trick
    pub fn play_trick(&mut self) -> (usize, i32) {
        self.trick.clear();

        for i in self.turn..(self.turn + 4) {
            let player_idx = i % 4;
            let card = self.players[player_idx].decide(self);
            let card_idx = card.get_index();

            //println!("player {} plays card {}", player_idx, card);

            debug_assert!(self.is_legal(card, player_idx));

            self.players[player_idx].toggle_card(card_idx);
            self.played_cards |= 1 << card_idx;
            self.trick.play(card, player_idx);
        }

        let winner = self.trick.winning_player().unwrap();
        let winning_team = winner % 2;
        // the winner of a trick get's to play first in the next trick
        self.turn = winner;
        //println!("player {} of team {} won", winner, winning_team);

        (winning_team, self.trick.score())
    }

    /// play an entire round, i.e. 8 tricks
    /// this method also assigns the next dealer
    pub fn play_round(&mut self) {
        let trump = self.players[self.dealer].pick_trump(self);
        //println!("player {} picks {:?} as trump", self.dealer, trump);
        self.trick.set_trump(Some(trump));

        let mut scores = [0; 2];

        for _ in 0..8 {
            let (winning_team, score) = self.play_trick();
            scores[winning_team] += score;
        }

        assert!(scores.iter().sum::<i32>() == 60);

        let (winning_team, &score) = scores.iter().enumerate().max_by_key(|(_, &y)| y).unwrap();
        self.score[winning_team] += score - 30;

        self.set_next_dealer();
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, card: Card, player: usize) -> bool {
        self.legal_actions(player).has_card(card)
        //if let Some(suite) = self.trick.suite_to_follow() {
        //    let player = &self.players[player];
        //
        //    !player.cards().has_suite(suite) || card.suite() == suite
        //} else {
        //    true
        //}
    }

    pub fn legal_actions(&self, player: usize) -> Stack {
        let mut cards = self.players[player].cards();

        // have to follow if possible,
        if let Some(suite) = self.trick.suite_to_follow() {
            let filtered_cards = cards & suite.mask();
            if filtered_cards != 0 {
                cards = filtered_cards;
            }
        }

        // this also means we're not the first player, i.e. the suite
        // to follow has been determined
        if let Some((winning_card, winning_player)) = self.trick.winner() {
            // our team is winning
            if winning_player % 2 == player % 2 {
                //todo!();
            } else {
                // have to buy if possible, but can't 'under-buy', except if that's our only possible move
                if let Some(trump) = self.trick.trump() {
                    let mut mask = Stack::all_above(winning_card) & winning_card.suite().mask();

                    // we can play any trump if the current winning card isn't a trump
                    if winning_card.suite() != trump {
                        mask |= trump.mask();
                    }

                    let filtered_cards = cards & mask;
                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
                // this means that we're playing without trump,
                // so we simply need to play a higher card of the same suite
                else {
                    let mask = Stack::all_above(winning_card) & winning_card.suite().mask();
                    let filtered_cards = cards & mask;

                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
            }
        }

        cards
    }

    pub fn is_terminal(&self) -> bool {
        self.score.iter().any(|&s| s >= 61)
    }

    pub fn winner(&self) -> usize {
        let (winning_team, _) = self
            .score
            .iter()
            .enumerate()
            .max_by_key(|(_, &y)| y)
            .unwrap();

        winning_team
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "dealer:\t{}", self.dealer)?;
        writeln!(f, "turn:\t{}", self.turn)?;
        writeln!(f, "score:\t{:?}", self.score)?;
        for i in 0..4 {
            writeln!(f, "player {i}: {:?}", self.players[i].cards())?;
        }
        Ok(())
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
    fn test_random_round() {
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
