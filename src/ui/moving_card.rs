use macroquad::{
    color::{GRAY, WHITE},
    math::{vec2, Vec2},
    miniquad::window::screen_size,
    texture::{draw_texture_ex, DrawTextureParams},
    time::get_frame_time,
};

use super::app::App;
use crate::card::Card;

const MAX_LIFE_TIME: f32 = 1.5;

pub struct MovingCard {
    pos: Vec2,
    start_pos: Vec2,
    card: Card,
    life_time: f32,
}

impl MovingCard {
    pub fn new(card: Card, start_pos: Vec2) -> Self {
        MovingCard {
            pos: start_pos,
            start_pos,
            card,
            life_time: 0.,
        }
    }

    pub fn draw(&self, app: &App) {
        let texture = &app.textures[&self.card.get_index()];
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            GRAY,
            DrawTextureParams {
                dest_size: Some(app.card_texture_size()),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self) -> bool {
        let (width, height) = screen_size();
        let end_pos = vec2(width * 0.4, height * 0.4);
        self.life_time += get_frame_time();
        self.pos = self.start_pos.lerp(end_pos, self.life_time / MAX_LIFE_TIME);

        self.life_time >= MAX_LIFE_TIME
    }
}
