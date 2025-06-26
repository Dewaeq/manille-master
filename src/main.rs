use io::arg_handler::handle_args;
#[cfg(feature = "log")]
use nn::logger::start_log;
use ui::app::App;

mod action;
mod action_collection;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod inference;
mod io;
mod nn;
mod players;
mod round;
mod sprt;
mod stack;
mod suit;
mod tournament;
mod trick;
mod ui;

#[cfg(not(feature = "log"))]
#[macroquad::main("main")]
async fn main() {
    romu::seed();
    let args: Vec<String> = std::env::args().collect();
    handle_args(args);

    let mut app = App::new().await;
    app.run().await;
}

#[cfg(feature = "log")]
fn main() {
    let log = start_log(15);
    println!("{}", log.len());
}
