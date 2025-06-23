use std::collections::HashMap;

use io::arg_handler::handle_args;
use macroquad::{
    miniquad::window::screen_size,
    prelude::*,
    ui::{hash, root_ui, widgets, Layout},
};
use round::Round;
use stack::Stack;
use ui::{app::App, card_texture};

mod action;
mod action_collection;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod inference;
mod io;
mod players;
mod round;
mod sprt;
mod stack;
mod suit;
mod tournament;
mod trick;
mod ui;

#[macroquad::main("main")]
async fn main() {
    romu::seed();
    // let args: Vec<String> = std::env::args().collect();
    // handle_args(args);

    let mut app = App::new().await;
    app.run().await;
}
