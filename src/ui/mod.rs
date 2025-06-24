use std::{collections::HashMap, sync::OnceLock};

use crate::{card::Card, stack::Stack, suit::Suit};
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

static TEXTURES: OnceLock<HashMap<u32, Texture2D>> = OnceLock::new();
const BOT_TEXTURE_OFFSET: u32 = 10000;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn card_texture(card: Card) -> Texture2D {
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

    let asset = Assets::get(&format!("cards/{name}")).unwrap();
    Texture2D::from_file_with_format(&asset.data, None)
}

pub fn load_textures() {
    let mut textures = HashMap::new();
    for card in Stack::ALL.into_iter() {
        textures.insert(card.get_index(), card_texture(card));
    }
    for i in 1..4 {
        let asset = Assets::get(&format!("bots/bot{i}.png")).unwrap();
        let texture = Texture2D::from_file_with_format(&asset.data, None);
        textures.insert(BOT_TEXTURE_OFFSET + i, texture);
    }
    TEXTURES.set(textures).unwrap();
}

pub fn get_card_texture(card: &Card) -> &Texture2D {
    TEXTURES.get().unwrap().get(&card.get_index()).unwrap()
}

pub fn get_bot_texture(index: &u32) -> &Texture2D {
    TEXTURES
        .get()
        .unwrap()
        .get(&(index + BOT_TEXTURE_OFFSET))
        .unwrap()
}

pub fn is_mouse_over(rect: Rect) -> bool {
    let (x, y) = mouse_position();
    (x >= rect.x) && (x <= rect.x + rect.w) && (y >= rect.y && y <= rect.y + rect.h)
}

pub fn get_card_size() -> Vec2 {
    let card = Card::new(0);
    let texture = get_card_texture(&card);
    let aspect_ratio = texture.height() / texture.width();
    let width = screen_width() * 0.07;
    vec2(width, aspect_ratio * width)
}
