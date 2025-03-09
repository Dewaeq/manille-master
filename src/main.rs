use io::arg_handler::handle_args;
use ui::app::App;

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
mod suit;
mod tournament;
mod trick;
mod ui;

fn main() {
    romu::seed();
    env_logger::init();

    let options = eframe::NativeOptions::default();

    let app = eframe::run_native(
        App::name(),
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    );

    let args: Vec<String> = std::env::args().collect();
    handle_args(args);
}
