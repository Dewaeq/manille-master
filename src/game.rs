use std::fmt::Debug;

use ismcts::{action_list::ActionList, state::State};

use crate::{action::Action, inference::Inference, players::PlayerVec, round::Round, stack::Stack};

const MAX_SCORE: i16 = 61;

#[derive(Default)]
pub struct Game {
    players: PlayerVec,
    round: Round,
    inference: Inference,
    num_rounds: usize,
    scores: [i16; 2],
}

impl Game {
    pub fn new(players: PlayerVec) -> Self {
        let dealer = romu::range_usize(0..4);

        Game {
            players,
            round: Round::new(dealer),
            inference: Default::default(),
            num_rounds: 0,
            scores: [0; 2],
        }
    }

    fn apply_action(&mut self, action: Action) {
        self.inference.infer(&self.round, action, self.round.turn());
        self.round.apply_action(action);
    }

    fn play_trick(&mut self) {
        let turn = self.round.turn();

        for i in turn..(turn + 4) {
            let player_idx = i % 4;
            let action = self.players[player_idx].decide(self.round, &self.inference);

            match action {
                Action::PlayCard(_) => {
                    debug_assert!(self.is_legal(action));

                    self.apply_action(action);
                }
                _ => unreachable!(),
            }
        }
    }

    /// play an entire round, i.e. 8 tricks
    pub fn play_round(&mut self) {
        self.inference = Inference::default();
        self.round.setup_for_next_round();

        let action = self.players[self.round.turn()].decide(self.round, &self.inference);
        self.apply_action(action);

        for _ in 0..8 {
            self.play_trick();
        }

        let scores = self.round.scores();
        let winning_team = if scores[0] > scores[1] { 0 } else { 1 };
        self.scores[winning_team] += scores[winning_team] - 30;

        assert!(scores.iter().sum::<i16>() == 60);
        self.num_rounds += 1;
    }

    /// controleer of deze speler al dan niet kan volgen
    pub fn is_legal(&self, action: Action) -> bool {
        self.legal_actions().has(&action)
    }

    pub fn legal_actions(&self) -> <Round as State>::ActionList {
        self.round.possible_actions()
    }

    pub const fn player_cards(&self, player: usize) -> Stack {
        self.round.player_cards(player)
    }

    pub const fn num_rounds(&self) -> usize {
        self.num_rounds
    }

    pub fn is_terminal(&self) -> bool {
        self.scores.iter().any(|&s| s >= MAX_SCORE)
    }

    pub fn winner(&self) -> usize {
        assert!(self.is_terminal());

        self.scores
            .iter()
            .enumerate()
            .max_by_key(|(_, s)| **s)
            .unwrap()
            .0
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "total score: {:?}", self.scores)?;
        writeln!(f, "rounds: {}", self.num_rounds)?;
        writeln!(f, "{:?}", self.round)
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

        for i in 0..4 {
            let cards = game.player_cards(i);
            seen_cards |= cards;

            assert!(cards.len() == Stack::ALL.len() / (game.players.len() as u32));
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
        while !game.is_terminal() {
            game.play_round();
        }
    }
}
