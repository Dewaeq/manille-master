use std::collections::HashMap;

use macroquad::{
    color::WHITE,
    math::{vec2, Vec2},
    miniquad::window::screen_size,
    texture::Texture2D,
    ui::{hash, root_ui, widgets},
    window::{clear_background, next_frame, screen_width},
};

use crate::{round::Round, stack::Stack, ui::card_texture};

use super::{hand::Hand, moving_card::MovingCard};

pub struct App {
    pub round: Round,
    pub textures: HashMap<u32, Texture2D>,
    moving_cards: Vec<MovingCard>,
}

impl App {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();
        for card in Stack::ALL.into_iter() {
            textures.insert(card.get_index(), card_texture(card).await);
        }

        App {
            round: Round::new(0),
            textures,
            moving_cards: vec![],
        }
    }

    pub async fn run(&mut self) {
        let mut cards = self.round.player_cards(0);
        loop {
            clear_background(WHITE);
            let (width, height) = screen_size();

            widgets::Group::new(hash!(), vec2(width, height * 0.1))
                .position(vec2(0., 0.))
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "top status bar");
                });
            widgets::Group::new(hash!(), vec2(width, height * 0.6))
                .position(vec2(0., height * 0.1))
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "main");
                });

            if let Some(card) = Hand::draw(cards, self, &mut root_ui()) {
                cards.remove(card);
                let start_pos = vec2(0., height * 0.6);
                self.moving_cards.push(MovingCard::new(card, start_pos));
            }

            for moving_card in &self.moving_cards {
                moving_card.draw(self);
            }
            let mut to_remove = vec![];
            for (i, moving_card) in self.moving_cards.iter_mut().enumerate() {
                if moving_card.update() {
                    to_remove.push(i);
                }
            }

            for x in to_remove {
                self.moving_cards.remove(x);
            }

            next_frame().await;
        }
    }

    pub fn card_texture_size(&self) -> Vec2 {
        let texture = &self.textures[&0];
        let aspect_ratio = texture.height() / texture.width();
        let width = screen_width() * 0.07;
        vec2(width, aspect_ratio * width)
    }
}
