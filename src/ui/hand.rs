use crate::{card::Card, stack::Stack};
use macroquad::{
    miniquad::window::screen_size,
    prelude::*,
    ui::{hash, widgets, Ui},
};

use super::app::App;

pub struct Hand {}

impl Hand {
    pub fn draw(cards: Stack, app: &App, ui: &mut Ui) -> Option<Card> {
        let mut clicked_card = None;
        let (width, height) = screen_size();
        widgets::Group::new(hash!(), vec2(width, height * 0.3))
            .position(vec2(0., height * 0.7))
            .ui(ui, |ui| {
                for (i, card) in cards.into_iter().enumerate() {
                    let texture = app.textures[&card.get_index()].clone();
                    let size = app.card_texture_size();
                    let clicked = widgets::Button::new(texture)
                        .size(size)
                        .position(vec2(size.x * (i as f32), 0.))
                        .ui(ui);
                    if clicked {
                        clicked_card = Some(card);
                    }
                }
            });

        clicked_card
    }
}
