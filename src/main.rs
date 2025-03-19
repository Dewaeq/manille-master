use io::arg_handler::handle_args;
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
mod players;
mod round;
mod sprt;
mod stack;
mod suit;
mod tournament;
mod trick;
mod ui;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    romu::seed();

    if args.len() == 1 {
        let options = eframe::NativeOptions::default();

        eframe::run_native(
            App::name(),
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::<App>::default())
            }),
        )
    } else {
        handle_args(args);
        Ok(())
    }
}
