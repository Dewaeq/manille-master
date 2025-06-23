use macroquad::{
    color::{GRAY, WHITE},
    input::{is_mouse_button_pressed, MouseButton},
    math::{vec2, Rect, Vec2},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time::get_frame_time,
    window::screen_height,
};

use super::{app::TEXTURES, get_card_size, is_mouse_over};
use crate::card::Card;

const SPEED: f32 = 250.;

pub struct UiCard {
    pub card: Card,
    pub pos: Vec2,
    pub target_pos: Option<Vec2>,
    pub is_button: bool,
    pub is_disabled: bool,
    pub is_moving: bool,
    texture: Texture2D,
}

impl UiCard {
    pub fn new(card: Card, pos: Vec2, is_button: bool) -> Self {
        UiCard {
            card,
            pos,
            target_pos: None,
            is_moving: false,
            is_button,
            is_disabled: false,
            texture: TEXTURES.get().unwrap()[&card.get_index()].clone(),
        }
    }

    pub fn draw(&self) {
        let size = get_card_size();
        let color = if self.is_disabled { GRAY } else { WHITE };
        let draw_pos = if self.is_button && is_mouse_over(self.rect()) {
            self.pos + vec2(0., -size.y * 0.05)
        } else {
            self.pos
        };

        draw_texture_ex(
            &self.texture,
            draw_pos.x,
            draw_pos.y,
            color,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self) {
        if let Some(target) = self.target_pos {
            let delta = get_frame_time();
            let diff = target - self.pos;
            let dist = diff.length();
            if dist < 5. {
                self.is_moving = false;
                return;
            }
            self.pos += delta * SPEED * diff / dist * dist.clamp(1., 5.) * screen_height() / 1080.;
        }
    }

    pub fn clicked(&self) -> bool {
        self.is_button
            && !self.is_disabled
            && is_mouse_button_pressed(MouseButton::Left)
            && is_mouse_over(self.rect())
    }

    fn rect(&self) -> Rect {
        let size = get_card_size();
        Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: size.x,
            h: size.y,
        }
    }
}
