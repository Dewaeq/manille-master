use crate::{card::Card, suit::Suit};
use app::TEXTURES;
use macroquad::{
    input::mouse_position,
    math::{vec2, Rect, Vec2},
    texture::{load_texture, Texture2D},
    window::screen_width,
};

pub mod app;
pub mod hand;
pub mod moving_card;
pub mod ui_card;
pub mod ui_game;

pub async fn card_texture(card: Card) -> Texture2D {
    let mut name = match card.value() {
        0 => "7",
        1 => "8",
        2 => "9",
        3 => "jack",
        4 => "queen",
        5 => "king",
        6 => "ace",
        7 => "10",
        _ => panic!(),
    }
    .to_owned();
    name += "_of_";
    name += match card.suit() {
        Suit::Clubs => "clubs",
        Suit::Spades => "spades",
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
    };
    name += ".png";

    load_texture(&format!("assets/cards/{name}")).await.unwrap()
}

pub fn is_mouse_over(rect: Rect) -> bool {
    let (x, y) = mouse_position();
    (x >= rect.x) && (x <= rect.x + rect.w) && (y >= rect.y && y <= rect.y + rect.h)
}

pub fn get_card_size() -> Vec2 {
    let texture = &TEXTURES.get().unwrap()[&0];
    let aspect_ratio = texture.height() / texture.width();
    let width = screen_width() * 0.07;
    vec2(width, aspect_ratio * width)
}
