use io::arg_handler::handle_args;

mod action;
mod action_collection;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod io;
mod players;
mod round;
mod sprt;
mod stack;
mod suite;
mod tournament;
mod trick;

fn main() {
    romu::seed();

    let args: Vec<String> = std::env::args().collect();
    handle_args(args);
}
