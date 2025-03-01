use std::fmt::{Debug, Display};

use crate::{
    action::Action, action_collection::ActionCollection, card::Card, game_phase::GamePhase,
    mcts::state::State, stack::Stack, suite::Suite, trick::Trick,
};

const MAX_SCORE: i16 = 101;

#[derive(Clone, Default)]
pub struct GameState {
    turn: usize,
    dealer: usize,
    player_cards: [Stack; 4],
    played_cards: Stack,
    trick: Trick,
    /// total score over all rounds
    total_score: [i16; 2],
    round_score: [i16; 2],
    phase: GamePhase,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState::default();
        state.set_random_dealer();
        state.deal_cards();

        state
    }

    fn deal_cards(&mut self) {
        // TODO: replace
        //let mut indices: [u32; 32] = std::array::from_fn(|i| i as u32);
        let cards_to_deal = Stack::ALL;
        let mut indices = (0..32)
            .filter(|&x| cards_to_deal.has_index(x))
            .collect::<Vec<_>>();
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

    /// this also changes the game phase to [PickingTrump]
    fn set_random_dealer(&mut self) {
        self.dealer = romu::mod_usize(4);
        // TODO: replace
        // self.dealer = 0;
        self.turn = (self.dealer + 1) % 4;
        self.phase = GamePhase::PickingTrump;
    }

    /// this also changes the game phase to [PickingTrump]
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

        self.round_score[winning_team] += self.trick.score() as i16;
        self.turn = winner;
        self.trick.clear();

        if self.played_cards == Stack::ALL {
            self.on_round_finish();
        }
    }

    fn on_round_finish(&mut self) {
        debug_assert!(self.round_score.iter().sum::<i16>() == 60);

        let winning_team = if self.round_score[0] >= self.round_score[1] {
            0
        } else {
            1
        };

        self.total_score[winning_team] += self.round_score[winning_team] - 30;
        self.round_score = [0; 2];
        self.played_cards = Stack::default();

        if self.total_score[winning_team] >= MAX_SCORE {
            self.phase = GamePhase::Finished { winning_team };
        } else {
            self.set_next_dealer();
            self.deal_cards();
        }
    }

    pub const fn last_moved(&self) -> usize {
        (self.turn + 3) % 4
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
        let cards = self.player_cards[self.dealer];
        let mut bits = 0;

        for suite in [Suite::Pijkens, Suite::Klavers, Suite::Harten, Suite::Koeken] {
            if cards.has_suite(suite) {
                bits |= 1 << suite as u8;
            }
        }

        ActionCollection::Trumps(bits)
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

        ActionCollection::Cards(cards)
    }
}

impl State for GameState {
    type Action = Action;
    type ActionList = ActionCollection;

    /// returns either [self.dealer] or [self.turn], depending
    /// on the phase of the game
    /// will panic when called on a terminal state
    fn turn(&self) -> usize {
        match self.phase {
            GamePhase::PickingTrump => self.dealer,
            GamePhase::PlayingRound => self.turn,
            GamePhase::Finished { .. } => unreachable!(),
        }
    }

    fn randomize(&self, observer: usize) -> Self {
        //println!("original state:");
        //dbg!(&self);

        let mut state = self.clone();
        let cards_to_deal = Stack::ALL ^ self.player_cards[observer] ^ self.played_cards;
        let mut indices = (0..32)
            .filter(|&x| cards_to_deal.has_index(x))
            .collect::<Vec<_>>();

        romu::shuffle(&mut indices);
        let mut start = 0;

        for i in 1..=3 {
            let n = self.player_cards[(observer + i) % 4].len() as usize;
            state.player_cards[(observer + i) % 4] =
                Stack::from_slice(&indices[start..(start + n)]);
            start += n;
        }

        // TODO: remove
        for i in 0..4 {
            assert_eq!(self.player_cards[i].len(), state.player_cards[i].len());
            if i != observer {
                //assert_ne!(self.player_cards[i], state.player_cards[i]);
            } else {
                assert_eq!(self.player_cards[i], state.player_cards[i]);
            }
        }

        //let mut cards = [Stack::default(); 2];
        // number of cards per player
        //let n = indices.len() / 4;
        //
        //for i in (2 * n..4 * n).rev() {
        //    let j = romu::mod_usize(i + 1);
        //    indices.swap(i, j);
        //
        //    cards[(i / n) - 2] |= 1 << indices[i];
        //}

        //state.player_cards[(observer + 1) % 4] = cards[0];
        //state.player_cards[(observer + 2) % 4] = cards[1];
        //state.player_cards[(observer + 3) % 4] = cards_to_deal ^ cards[0] ^ cards[1];

        //dbg!(observer);
        //dbg!(&state);

        state
    }

    /// TODO: find a better way to do this, cus this sucks
    fn empty_action_list(&self) -> Self::ActionList {
        match self.phase {
            GamePhase::PickingTrump => ActionCollection::Trumps(0),
            GamePhase::PlayingRound => ActionCollection::Cards(Stack::default()),
            _ => ActionCollection::Trumps(0),
        }
    }

    /// return possible cards to play by [turn],
    /// or possible trumps to pick by [dealer]
    fn possible_actions(&self) -> Self::ActionList {
        match self.phase {
            GamePhase::PickingTrump => self.possible_trump_actions(),
            GamePhase::PlayingRound => self.possible_card_actions(),
            _ => unreachable!(),
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

    fn reward(&self, perspective: usize) -> f32 {
        match self.phase {
            GamePhase::Finished { winning_team } => {
                if perspective % 2 == winning_team {
                    1.
                } else {
                    0.
                }
            }
            _ => unreachable!(),
        }
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
            //.field("player_cards", &self.player_cards)
            .field("played_cards", &self.played_cards)
            .field("trick", &self.trick)
            .field("score", &self.total_score)
            .field("trick_score", &self.round_score)
            .field("phase", &self.phase)
            .finish()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "player {i}: {:?}", self.player_cards[i])?;
        }

        writeln!(f, "phase: {:?}", self.phase)?;
        writeln!(f, "dealer: {:?}", self.dealer)?;
        writeln!(f, "turn: {:?}", self.turn)?;
        writeln!(f, "total score: {:?}", self.total_score)?;

        Ok(())
    }
}
