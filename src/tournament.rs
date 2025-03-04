use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{game::Game, players::PlayerVec};

#[derive(Default, Debug)]
struct TournamentResult {
    scores: [i32; 2],
    num_games: usize,
    num_rounds: usize,
    duration: Duration,
}

pub fn run_tournament_multithreaded(
    num_games: usize,
    num_threads: usize,
    name: &str,
    verbose: bool,
    player_gen: impl Fn() -> PlayerVec + std::marker::Send + Clone,
) {
    println!(
        "starting tournament '{name}' with {num_threads} threads each playing {num_games} games"
    );

    let started = Instant::now();
    let results = Arc::new(Mutex::new(TournamentResult::default()));

    std::thread::scope(|s| {
        for _ in 0..num_threads {
            let results = Arc::clone(&results);
            let player_gen = player_gen.clone();

            s.spawn(move || {
                let thread_result = run_tournament(num_games, verbose, player_gen);

                let mut results = results.lock().unwrap();
                results.scores[0] += thread_result.scores[0];
                results.scores[1] += thread_result.scores[1];
                results.num_rounds += thread_result.num_rounds;
                results.num_games += thread_result.num_games;
            });
        }
    });

    results.lock().unwrap().duration = started.elapsed();
    println!("{name}:\n{:?}", results.lock().unwrap());
}

fn run_tournament(
    num_games: usize,
    verbose: bool,
    player_gen: impl Fn() -> PlayerVec,
) -> TournamentResult {
    let mut games = Vec::with_capacity(num_games);

    for _ in 0..num_games {
        games.push(Game::new(player_gen()));
    }

    let start = Instant::now();
    let mut total_rounds = 0;

    for game in &mut games {
        while !game.is_terminal() {
            game.play_round();
        }

        total_rounds += game.num_rounds();
    }

    let mut score = [0; 2];
    for game in &mut games {
        score[game.winner()] += 1;
    }

    TournamentResult {
        scores: score,
        num_rounds: total_rounds,
        num_games,
        duration: start.elapsed(),
    }
}
