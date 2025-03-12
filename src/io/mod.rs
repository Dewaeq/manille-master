use crate::{card::Card, suit::Suit};

pub mod arg_handler;
mod debugger;
mod input;

pub fn card_image_src(card: &Card) -> String {
    let mut result = "file://assets/cards/".to_string();
    let card_name = match card.value() {
        v @ 0..=2 => (v + 7).to_string(),
        3 => "jack".to_string(),
        4 => "queen".to_string(),
        5 => "king".to_string(),
        6 => "ace".to_string(),
        7 => "10".to_string(),
        _ => unreachable!(),
    };
    result.push_str(&card_name);

    result.push_str("_of_");
    result.push_str(match card.suit() {
        Suit::Clubs => "clubs",
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
        Suit::Spades => "spades",
    });
    result.push_str(".png");

    result
}
