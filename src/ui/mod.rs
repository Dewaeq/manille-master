use crate::{card::Card, suit::Suit};
use macroquad::texture::{load_texture, Texture2D};

pub mod app;
pub mod hand;
pub mod moving_card;

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
