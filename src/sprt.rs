//! NOTE: this is a very crude SPRT-like method,
//! which might very well be totally incorrect
use std::sync::{Arc, Mutex};

use crate::{game::Game, players::PlayerVec};

pub fn run_sprt(num_threads: usize, player_gen: impl Fn() -> PlayerVec + Clone + Send + Sync) {
    let num_wins = Arc::new(Mutex::new([0.; 2]));
    let num_games = Arc::new(Mutex::new(0.));

    std::thread::scope(|s| {
        for _ in 0..num_threads {
            let player_gen = player_gen.clone();
            let num_wins = Arc::clone(&num_wins);
            let num_games = Arc::clone(&num_games);

            s.spawn(move || loop {
                let mut game = Game::new(player_gen());
                while !game.is_terminal() {
                    game.play_round();
                }

                num_wins.lock().unwrap()[game.winner()] += 1.;
                *num_games.lock().unwrap() += 1.;

                let w = *num_wins.lock().unwrap();
                let n = *num_games.lock().unwrap();

                let ci_a = confidence_interval(w[0], n);
                let ci_b = confidence_interval(w[1], n);

                println!("win rates: a={}%, b={}%", w[0] / n * 100., w[1] / n * 100.);
                let mean = w[0] / n;
                let variance = mean * (1. - mean) / n;
                println!("sigma: {}", variance.sqrt());

                if n < 100. {
                    continue;
                }

                if ci_a.0 > ci_b.1 {
                    break Some(0);
                } else if ci_b.0 > ci_a.1 {
                    break Some(1);
                }
            });
        }
    });

    println!("games: {:?}", num_games.lock().unwrap());
    println!("wins: {:?}", num_wins.lock().unwrap());
}

fn confidence_interval(num_wins: f32, num_games: f32) -> (f32, f32) {
    const Z: f32 = 3.;
    let mean = num_wins / num_games;
    let variance = mean * (1. - mean) / num_games;
    let sigma = variance.sqrt();

    (mean - Z * sigma, mean + Z * sigma)
}
