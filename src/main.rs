use bench::bench;
use game::Game;

mod array;
mod bench;
mod bits;
mod card;
mod game;
mod human_player;
mod player;
mod random_player;
mod trick;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"bench".to_owned()) {
        let size = args.last().and_then(|x| x.parse::<usize>().ok());
        bench(size);

        return;
    }
}
