use std::fmt::Debug;

use ismcts::state::State;
use rand::seq::IndexedRandom;

use crate::{
    action::Action, action_collection::ActionCollection, card::Card, inference::Inference,
    stack::Stack, suit::Suit, trick::Trick,
};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundPhase {
    #[default]
    PickTrump,
    PlayCards,
}

#[derive(Default, Clone, Copy)]
pub struct Round {
    turn: usize,
    dealer: usize,
    player_cards: [Stack; 4],
    played_cards: Stack,
    scores: [i16; 2],
    trick: Trick,
    phase: RoundPhase,
}

impl Round {
    /// should only be used between tricks
    #[allow(clippy::too_many_arguments)]
    pub fn from_observer(
        observer_cards: Stack,
        played_cards: Stack,
        player_card_counts: [usize; 4],
        dealer: usize,
        turn: usize,
        phase: RoundPhase,
        trump: Option<Suit>,
        scores: [i16; 2],
    ) -> Self {
        let observer = 0;
        let mut round = Round::default().randomize_for(
            observer,
            observer_cards,
            played_cards,
            player_card_counts,
            &Inference::default(),
        );

        round.turn = turn;
        round.dealer = dealer;
        round.phase = phase;
        round.scores = scores;
        round.trick.set_trump(trump);

        round
    }

    pub fn observe_action(&self, observer: usize, action: Action, inference: &Inference) -> Self {
        let mut round = *self;
        if let Action::PlayCard(card) = action {
            if !round.player_cards(round.turn()).has_card(card) {
                round.player_cards[round.turn()].pop_random_card();
            }
        }

        round.apply_action(action);
        round.randomize(observer, inference)
    }

    fn randomize_for(
        &self,
        observer: usize,
        observer_cards: Stack,
        played_cards: Stack,
        mut player_card_counts: [usize; 4],
        inference: &Inference,
    ) -> Self {
        let mut round = *self;
        let mut cards_to_deal = Stack::ALL ^ observer_cards ^ played_cards;
        let mut players = Vec::with_capacity(4);

        for i in 0..4 {
            round.player_cards[i].clear();
        }

        let mut rng = rand::rng();
        while let Some(card) = cards_to_deal.pop_lowest() {
            for i in 1..=3 {
                let player = (observer + i) % 4;
                if player_card_counts[player] > 0 {
                    players.push(player);
                }
            }
            let chosen_player = players
                .choose_weighted(&mut rng, |&player| inference.weight(player, card))
                .unwrap_or(players.choose(&mut rng).unwrap());

            round.player_cards[*chosen_player].push(card);
            player_card_counts[*chosen_player] -= 1;
            players.clear();
        }

        round.player_cards[observer] = observer_cards;

        round
    }

    pub fn new(dealer: usize) -> Self {
        let mut round = Round::default();

        round.set_dealer(dealer);
        round.deal_cards();

        round
    }

    const fn set_dealer(&mut self, dealer: usize) {
        self.dealer = dealer;
        self.turn = (dealer + 1) % 4;
    }

    pub fn setup_for_next_round(&mut self) {
        let next_dealer = (self.dealer + 1) % 4;
        self.set_dealer(next_dealer);
        self.deal_cards();

        self.played_cards = Stack::default();
        self.scores = [0; 2];
        self.trick.clear();
        self.phase = RoundPhase::PickTrump;
    }

    fn deal_cards(&mut self) {
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

    fn play_card(&mut self, card: Card) {
        self.trick.play(card, self.turn);
        self.played_cards |= 1 << card.get_index();
        self.player_cards[self.turn] &= !(1 << card.get_index());

        if self.trick.is_finished() {
            self.on_trick_finish();
        } else {
            self.turn = (self.turn + 1) % 4;
        }
    }

    const fn set_trump(&mut self, trump: Option<Suit>) {
        self.trick.set_trump(trump);
        self.phase = RoundPhase::PlayCards;
    }

    const fn on_trick_finish(&mut self) {
        let (_, winner) = self.trick.winner().unwrap();
        let winning_team = winner % 2;

        self.scores[winning_team] += self.trick.score();
        self.turn = winner;
        self.trick.clear();
    }

    fn possible_card_actions(&self) -> <Self as State>::ActionList {
        let mut cards = self.player_cards[self.turn];

        // have to follow if possible,
        if let Some(suit) = self.trick.suit_to_follow() {
            let filtered_cards = cards & suit.mask();
            if filtered_cards != 0 {
                cards = filtered_cards;
            }
        }

        // this also means we're not the first player, i.e. the suit
        // to follow has been determined
        if let Some((winning_card, winning_player)) = self.trick.winner() {
            // our team isn't winning
            if winning_player % 2 != self.turn % 2 {
                // have to buy if possible, but can't 'under-buy', except if that's our only possible move
                if let Some(trump) = self.trick.trump() {
                    let mut mask = Stack::all_above(winning_card) & winning_card.suit().mask();

                    // we can play any trump if the current winning card isn't a trump
                    if winning_card.suit() != trump {
                        mask |= trump.mask();
                    }

                    let filtered_cards = cards & mask;
                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
                // this means that we're playing without trump,
                // so we simply need to play a higher card of the same suit
                else {
                    let mask = Stack::all_above(winning_card) & winning_card.suit().mask();
                    let filtered_cards = cards & mask;

                    if filtered_cards != 0 {
                        cards = filtered_cards;
                    }
                }
            }
        }

        ActionCollection::Cards(cards)
    }

    fn possible_trump_actions(&self) -> <Self as State>::ActionList {
        let cards = self.player_cards[self.dealer];
        let mut bits = 1 << 4;

        for suit in [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds] {
            if cards.has_suit(suit) {
                bits |= 1 << suit as u8;
            }
        }

        ActionCollection::Trumps(bits)
    }

    pub const fn player_cards(&self, player: usize) -> Stack {
        self.player_cards[player]
    }

    pub fn unplayed_cards(&self) -> Stack {
        !self.played_cards
    }

    pub const fn played_cards(&self) -> Stack {
        self.played_cards
    }

    pub const fn dealer(&self) -> usize {
        self.dealer
    }

    pub const fn phase(&self) -> RoundPhase {
        self.phase
    }

    pub const fn trump(&self) -> Option<Suit> {
        self.trick.trump()
    }

    pub fn suit_to_follow(&self) -> Option<Suit> {
        self.trick.suit_to_follow()
    }

    pub const fn trick_ref(&self) -> &Trick {
        &self.trick
    }

    pub const fn scores(&self) -> [i16; 2] {
        self.scores
    }
}

impl State for Round {
    type Action = Action;
    type ActionList = ActionCollection;
    type Inference = Inference;

    fn turn(&self) -> usize {
        match self.phase {
            RoundPhase::PickTrump => self.dealer,
            RoundPhase::PlayCards => self.turn,
        }
    }

    fn randomize(&self, observer: usize, inference: &Self::Inference) -> Self {
        let observer_cards = self.player_cards[observer];
        let player_card_counts = std::array::from_fn(|i| self.player_cards[i].len() as usize);

        self.randomize_for(
            observer,
            observer_cards,
            self.played_cards,
            player_card_counts,
            inference,
        )
    }

    fn possible_actions(&self) -> Self::ActionList {
        match self.phase {
            RoundPhase::PickTrump => self.possible_trump_actions(),
            RoundPhase::PlayCards => self.possible_card_actions(),
        }
    }

    fn apply_action(&mut self, action: Self::Action) {
        match action {
            Action::PlayCard(card) => self.play_card(card),
            Action::PickTrump(trump) => self.set_trump(trump),
        }
    }

    fn is_terminal(&self) -> bool {
        self.played_cards == Stack::ALL
    }

    fn reward(&self, perspective: usize) -> f32 {
        assert!(self.is_terminal());

        let team = perspective % 2;
        (self.scores[team] - 30) as f32 / 30.
    }
}

impl Debug for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "player {i}: {:?}", self.player_cards[i])?;
        }

        f.debug_struct("Round")
            .field("turn", &self.turn)
            .field("dealer", &self.dealer)
            .field("played_cards", &self.played_cards)
            .field("trick", &self.trick)
            .field("scores", &self.scores)
            .field("phase", &self.phase)
            .finish()
    }
}
