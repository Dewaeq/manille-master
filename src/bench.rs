use std::time::Instant;

use crate::{
    game::Game,
    players::{greedy_player::GreedyPlayer, random_player::RandomPlayer, Player, PlayerVec},
    stack::Stack,
};

const BENCH_SIZE: usize = 800_000;

pub fn bench(size: Option<usize>) {
    let size = size.unwrap_or(BENCH_SIZE);

    run_bench::<RandomPlayer>(size, "random");
    //run_bench::<GreedyPlayer>(size, "greedy");
}

fn run_bench<T: Player + Default + 'static>(size: usize, name: &str) {
    println!("Simulating {size} random games for {name}...");
    let mut games = Vec::with_capacity(size);

    for _ in 0..size {
        let players: PlayerVec = vec![T::boxed(), T::boxed(), T::boxed(), T::boxed()];

        games.push(Game::new(players));
    }

    let start = Instant::now();

    for game in &mut games {
        loop {
            game.play_round();
            if game.is_terminal() {
                break;
            }
            game.deal_cards();
        }
    }

    println!(
        "{name}:\t{}ms, {} games/s",
        start.elapsed().as_millis(),
        (size as f64) / start.elapsed().as_secs_f64()
    );

    let mut score = [0; 2];
    for game in &mut games {
        score[game.winner()] += 1;
    }

    println!("{name}:\t{score:?}");
}
