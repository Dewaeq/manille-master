use std::fmt::Debug;

use crate::{
    action::Action, card::Card, game_phase::GamePhase, mcts::state::State, stack::Stack,
    suite::Suite, trick::Trick,
};

#[derive(Clone, Default)]
pub struct GameState {
    turn: usize,
    dealer: usize,
    player_cards: [Stack; 4],
    played_cards: Stack,
    trick: Trick,
    /// total score over all rounds
    score: [i16; 2],
    trick_score: [i16; 2],
    phase: GamePhase,
}

impl GameState {
    pub fn deal_cards(&mut self) {
        let mut indices: [u32; 32] = std::array::from_fn(|i| i as u32);
        let mut cards = [Stack::default(); 3];

        // number of cards per player
        let n = indices.len() / 4;

        for i in (n..4 * n).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[(i / n) - 1] |= 1 << indices[i];
        }

        self.player_cards[0] = cards[0];
        self.player_cards[1] = cards[1];
        self.player_cards[2] = cards[2];
        self.player_cards[3] = Stack::ALL ^ cards[0] ^ cards[1] ^ cards[2];
    }

    pub fn set_random_dealer(&mut self) {
        self.dealer = romu::mod_usize(4);
        self.turn = (self.dealer + 1) % 4;
    }

    fn set_next_dealer(&mut self) {
        self.dealer = (self.dealer + 1) % 4;
        self.turn = (self.dealer + 1) % 4;
        self.phase = GamePhase::PickingTrump;
    }

    fn play_card(&mut self, card: Card) {
        self.trick.play(card, self.turn);
        self.played_cards |= 1 << card.get_index();
        self.player_cards[self.turn] ^= 1 << card.get_index();

        if self.trick.is_finished() {
            self.on_trick_finish();
        } else {
            self.turn = (self.turn + 1) % 4;
        }
    }

    fn set_trump(&mut self, trump: Option<Suite>) {
        self.trick.set_trump(trump);
        self.phase = GamePhase::PlayingRound;
    }

    fn on_trick_finish(&mut self) {
        let winner = self.trick.winning_player().unwrap();
        let winning_team = winner % 2;

        self.trick_score[winning_team] += self.trick.score() as i16;
        self.turn = winner;
        self.trick.clear();

        if self.played_cards == Stack::ALL {
            self.on_round_finish();
        }
    }

    fn on_round_finish(&mut self) {
        debug_assert!(self.trick_score.iter().sum::<i16>() == 60);

        let winning_team = if self.trick_score[0] >= self.trick_score[1] {
            0
        } else {
            1
        };

        self.score[winning_team] += self.trick_score[winning_team] - 30;
        self.trick_score = [0; 2];
        self.played_cards = Stack::default();

        if self.score[0] >= 61 {
            self.phase = GamePhase::Finished { winning_team: 0 };
        } else if self.score[1] >= 61 {
            self.phase = GamePhase::Finished { winning_team: 1 };
        } else {
            self.set_next_dealer();
        }
    }

    pub const fn turn(&self) -> usize {
        self.turn
    }

    pub const fn dealer(&self) -> usize {
        self.dealer
    }

    pub const fn cards(&self, player: usize) -> Stack {
        self.player_cards[player]
    }

    pub const fn winner(&self) -> usize {
        match self.phase {
            GamePhase::Finished { winning_team } => winning_team,
            _ => unreachable!(),
        }
    }

    /// TODO: add option to play without trump
    fn possible_trump_actions(&self) -> <Self as State>::ActionList {
        let cards = self.player_cards[self.turn];
        let mut actions = Vec::with_capacity(4);

        for suite in [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken] {
            if cards.has_suite(suite) {
                actions.push(Action::PickTrump(Some(suite)));
            }
        }

        actions
    }

    fn possible_card_actions(&self) -> <Self as State>::ActionList {
        let mut cards = self.player_cards[self.turn];

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
            if winning_player % 2 == self.turn % 2 {
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

        cards.into_iter().map(Action::PlayCard).collect()
    }
}

impl State for GameState {
    type Action = Action;
    type ActionList = Vec<Self::Action>;

    fn randomize(&self, observer: usize) -> Self {
        let mut state = self.clone();
        let cards_to_deal = Stack::ALL ^ self.player_cards[observer] ^ self.played_cards;
        let mut indices = (0..32)
            .filter(|&x| cards_to_deal.has_index(x))
            .collect::<Vec<_>>();
        let mut cards = [Stack::default(); 2];

        // number of cards per player
        let n = indices.len() / 4;

        for i in (2 * n..4 * n).rev() {
            let j = romu::mod_usize(i + 1);
            indices.swap(i, j);

            cards[(i / n) - 2] |= 1 << indices[i];
        }

        state.player_cards[(observer + 1) % 4] = cards[0];
        state.player_cards[(observer + 2) % 4] = cards[1];
        state.player_cards[(observer + 3) % 4] =
            Stack::ALL ^ cards[0] ^ cards[1] ^ self.player_cards[observer];

        state
    }

    fn possible_actions(&self) -> Self::ActionList {
        match self.phase {
            GamePhase::PickingTrump => self.possible_trump_actions(),
            GamePhase::PlayingRound => self.possible_card_actions(),
            _ => panic!(),
        }
    }

    fn apply_action(&mut self, action: Self::Action) {
        match action {
            Action::PlayCard(c) => self.play_card(c),
            Action::PickTrump(t) => self.set_trump(t),
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self.phase, GamePhase::Finished { .. })
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "player {i}: {:?}", self.player_cards[i])?;
        }

        f.debug_struct("GameState")
            .field("turn", &self.turn)
            .field("dealer", &self.dealer)
            .field("player_cards", &self.player_cards)
            .field("played_cards", &self.played_cards)
            //.field("trick", &self.trick)
            .field("score", &self.score)
            .field("trick_score", &self.trick_score)
            .field("phase", &self.phase)
            .finish()
    }
}
