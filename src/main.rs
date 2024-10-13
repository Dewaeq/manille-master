use std::time::Instant;

use card::Card;
use game::Game;

mod array;
mod bits;
mod card;
mod game;
mod human_player;
mod player;
mod random_player;
mod trick;

const N: usize = 80_000;

fn main() {
    assert!(Card::new(34, 0).to_index() == 34);
    assert!(Card::new(4, 3).to_index() == 4);
    assert!(Card::new(17, 2).to_index() == 17);

    let mut games = Vec::with_capacity(N);
    for _ in 0..N {
        games.push(Game::new());
    }

    let start = Instant::now();

    for game in &mut games {
        for _ in 0..13 {
            game.play_trick();
        }
    }

    println!(
        "{}ms, {} games/s",
        start.elapsed().as_millis(),
        (N as f64) / start.elapsed().as_secs_f64()
    );

    let mut score = [0; 4];
    for game in &mut games {
        for i in 0..4 {
            score[i] += game.score[i];
        }
    }

    println!("{score:?}");
}
