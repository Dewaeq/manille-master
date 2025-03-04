use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    game::Game,
    players::{mcts_player::MctsPlayer, random_player::RandomPlayer, Player, PlayerVec},
};

const BENCH_SIZE: usize = 800_000;
const THINK_TIME: u128 = 200;

pub fn bench(size: Option<usize>) {
    let size = size.unwrap_or(BENCH_SIZE);

    //start_simple_bench::<RandomPlayer>(size, "random");
    run_bench_multithreaded(14, size, "mcts vs random", false, || {
        vec![
            Box::new(MctsPlayer::default().set_search_time(THINK_TIME)),
            RandomPlayer::boxed(),
            Box::new(MctsPlayer::default().set_search_time(THINK_TIME)),
            RandomPlayer::boxed(),
        ]
    });
}

fn start_simple_bench<T: Player + Default + 'static>(size: usize, name: &str) {
    let player_gen = || -> PlayerVec { vec![T::boxed(), T::boxed(), T::boxed(), T::boxed()] };
    run_bench(size, name, false, player_gen);
}

fn run_bench_multithreaded(
    num_threads: usize,
    size: usize,
    name: &str,
    verbose: bool,
    player_gen: fn() -> PlayerVec,
) {
    let results = Arc::new(Mutex::new([0; 2]));

    std::thread::scope(|s| {
        for _ in 0..num_threads {
            let results = Arc::clone(&results);

            s.spawn(move || {
                let thread_result = run_bench(size, name, verbose, player_gen);

                let mut results = results.lock().unwrap();
                results[0] += thread_result[0];
                results[1] += thread_result[1];
            });
        }
    });

    println!("{name}:\t{:?}", results.lock().unwrap());
}

fn run_bench(size: usize, name: &str, verbose: bool, player_gen: fn() -> PlayerVec) -> [i32; 2] {
    println!("Simulating {size} random games for {name}...");

    let mut games = Vec::with_capacity(size);

    for _ in 0..size {
        games.push(Game::new(player_gen()));
    }

    let start = Instant::now();
    let mut total_rounds = 0;

    for game in &mut games {
        while !game.is_terminal() {
            game.play_round();
        }

        total_rounds += game.num_rounds();

        if verbose {
            println!("{}", game.winner());
        }
    }

    println!(
        "{name}:\t{}ms, {} games/s",
        start.elapsed().as_millis(),
        (size as f64) / start.elapsed().as_secs_f64()
    );
    println!("avg num of rounds: {}", total_rounds as f64 / size as f64);

    let mut score = [0; 2];
    for game in &mut games {
        score[game.winner()] += 1;
    }

    score
}
