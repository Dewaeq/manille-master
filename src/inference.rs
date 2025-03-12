use std::fmt::Debug;

use crate::{action::Action, card::Card, round::Round, suit::Suit};

#[derive(Default, Debug)]
pub struct Inference {
    players: [CardLikelihood; 4],
}

impl ismcts::inference::Inference for Inference {}

impl Inference {
    pub fn weights(&self, player: usize) -> [f32; 32] {
        self.players[player].l
    }

    pub fn weight(&self, player: usize, card: Card) -> f32 {
        self.players[player].l[card.get_index() as usize]
    }

    pub fn infer(&mut self, state: &Round, action: Action, player: usize) {
        match action {
            Action::PlayCard(card) => self.infer_card(state, player, card),
            Action::PickTrump(trump) => self.infer_trump(state, player, trump),
        }

        for player in &mut self.players {
            player.rescale();
        }
    }

    fn infer_card(&mut self, state: &Round, player: usize, card: Card) {
        for player in &mut self.players {
            player.remove_card(card);
        }

        let mut followed = true;
        if let Some(suit) = state.suit_to_follow() {
            if card.suit() != suit {
                followed = false;
                for card in state.unplayed_cards().of_suit(suit).into_iter() {
                    self.players[player].remove_card(card);
                }
            }
        }

        if let Some((winning_card, winning_player)) = state.trick_ref().winner() {
            // if the player is losing the trick and follows without buying,
            // that means they have no higher cards of that suit
            if winning_player % 2 != player % 2
                && winning_card.suit() == card.suit()
                && winning_card.value() > card.value()
            {
                for card in state
                    .unplayed_cards()
                    .of_suit(card.suit())
                    .above(winning_card)
                    .into_iter()
                {
                    self.players[player].remove_card(card);
                }
            }

            // likewise, if the player can't follow and doesn't
            // play a trump when no trump has been played yet, that means they're void of trumps
            if winning_player % 2 != player % 2
                && !followed
                && winning_card.suit() != card.suit()
                && state
                    .trump()
                    .is_some_and(|trump| card.suit() != trump && winning_card.suit() != trump)
            {
                let trump = state.trump().unwrap();

                for card in state.unplayed_cards().of_suit(trump).into_iter() {
                    self.players[player].remove_card(card);
                }
            }

            // if the player doesn't follow and the current winning card is a trump, which they don't beat,
            // than they don't have any higher trumps than the winning card
            if winning_player % 2 != player % 2
                && !followed
                && state
                    .trump()
                    .is_some_and(|trump| card.suit() != trump && winning_card.suit() == trump)
            {
                for card in state
                    .unplayed_cards()
                    .of_suit(winning_card.suit()) // i.e. trump
                    .above(winning_card)
                    .into_iter()
                {
                    self.players[player].remove_card(card);
                }
            }
        }
    }

    fn infer_trump(&mut self, state: &Round, player: usize, trump: Option<Suit>) {
        if let Some(suit) = trump {
            for (i, p) in self.players.iter_mut().enumerate() {
                for card in state.unplayed_cards().of_suit(suit).into_iter() {
                    let prob = (card.value() as f32 + 5.) / 12. * 0.7;
                    if i == player {
                        p.set_if_has(card, prob);
                    } else {
                        p.scale(card, 1. - prob);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct CardLikelihood {
    l: [f32; 32],
}

impl CardLikelihood {
    fn rescale(&mut self) {
        let sum = self.l.iter().sum::<f32>();
        for val in &mut self.l {
            *val /= sum;
        }
    }

    fn set_if_has(&mut self, card: Card, prob: f32) {
        let index = card.get_index() as usize;
        if self.l[index] != 0. {
            self.l[index] = prob;
        }
    }

    fn scale(&mut self, card: Card, factor: f32) {
        self.l[card.get_index() as usize] *= factor;
    }

    fn remove_card(&mut self, card: Card) {
        self.l[card.get_index() as usize] = 0.;
    }
}

impl Default for CardLikelihood {
    fn default() -> Self {
        Self { l: [1. / 32.; 32] }
    }
}

impl Debug for CardLikelihood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            write!(f, "{}: {:.5}\t", Card::new(i as u32), self.l[i])?;
            write!(f, "{}: {:.5}\t", Card::new(i as u32 + 8), self.l[i + 8])?;
            write!(f, "{}: {:.5}\t", Card::new(i as u32 + 16), self.l[i + 16])?;
            writeln!(f, "{}: {:.5}\t", Card::new(i as u32 + 24), self.l[i + 24])?;
        }

        Ok(())
    }
}
