use std::sync::LazyLock;
use eframe::egui::{self, include_image, Image};

pub mod app;

static CARD_IMAGES: LazyLock<Vec<Image<'static>>> = LazyLock::new(|| store_images());

fn store_images() -> Vec<Image<'static>> {
    let mut images = vec![];

    images.push(egui::Image::new(include_image!(
        "../../assets/cards/7_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/8_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/9_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/jack_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/queen_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/king_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/ace_of_spades.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/10_of_spades.png"
    )));

    images.push(egui::Image::new(include_image!(
        "../../assets/cards/7_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/8_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/9_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/jack_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/queen_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/king_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/ace_of_clubs.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/10_of_clubs.png"
    )));

    images.push(egui::Image::new(include_image!(
        "../../assets/cards/7_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/8_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/9_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/jack_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/queen_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/king_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/ace_of_hearts.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/10_of_hearts.png"
    )));

    images.push(egui::Image::new(include_image!(
        "../../assets/cards/7_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/8_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/9_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/jack_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/queen_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/king_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/ace_of_diamonds.png"
    )));
    images.push(egui::Image::new(include_image!(
        "../../assets/cards/10_of_diamonds.png"
    )));

    images
}
