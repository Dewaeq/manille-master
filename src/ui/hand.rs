use super::{get_card_size, ui_card::UiCard};
use crate::stack::Stack;
use macroquad::{miniquad::window::screen_size, prelude::*};

pub const SPACING_FACTOR: f32 = 1.07;

pub struct Hand {}

impl Hand {
    pub fn draw(cards: Stack, legal: Stack) -> Option<UiCard> {
        let mut clicked_card = None;
        let (width, height) = screen_size();
        let size = get_card_size();
        let padding = (width - size.x * SPACING_FACTOR * cards.len() as f32) * 0.5;
        for (i, card) in cards.into_iter().enumerate() {
            let pos = vec2(
                padding + size.x * SPACING_FACTOR * i as f32,
                height - size.y * 1.1,
            );
            let mut ui_card = UiCard::new(card, pos, true);
            ui_card.is_disabled = !legal.has_card(card);
            ui_card.draw();

            if ui_card.clicked() {
                clicked_card = Some(ui_card);
            }
        }

        clicked_card
    }
}
