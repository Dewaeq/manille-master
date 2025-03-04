use std::io::stdin;

use bench::bench;
use mcts::state::State;
use players::{mcts_player::MctsPlayer, random_player::RandomPlayer, Player, PlayerVec};
use round::Round;
use tournament::run_tournament_multithreaded;

mod action;
mod action_collection;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod game_phase;
mod mcts;
mod players;
mod round;
mod stack;
mod suite;
mod tournament;
mod trick;

fn main() {
    romu::seed();

    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"tournament".to_owned()) {
        let mut it = args.iter().skip(2);

        let num_games = it.next().and_then(|x| x.parse::<usize>().ok()).unwrap_or(5);
        let num_threads = it
            .next()
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(14);
        let think_time = it
            .next()
            .and_then(|x| x.parse::<u128>().ok())
            .unwrap_or(500);
        let verbose = args.contains(&"verbose".to_owned());

        let player_gen = move || -> PlayerVec {
            vec![
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                Box::new(RandomPlayer::default()),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                Box::new(RandomPlayer::default()),
            ]
        };

        run_tournament_multithreaded(
            num_games,
            num_threads,
            "self determ vs normal",
            verbose,
            player_gen,
        );
    }

    if args.contains(&"bench".to_owned()) {
        let size = args.last().and_then(|x| x.parse::<usize>().ok());
        bench(size);
    }

    if args.contains(&"d".to_owned()) {
        let mut state = Round::new(romu::range_usize(0..4));
        let mut player = MctsPlayer::default().set_search_time(1_000);
        let mut buf = String::new();

        loop {
            buf.clear();
            stdin().read_line(&mut buf).unwrap();
            for c in buf.chars() {
                match c {
                    'q' => return,
                    '+' => {
                        let prev_time = player.get_search_time();
                        player = MctsPlayer::default().set_search_time(prev_time + 100);
                    }
                    '-' => {
                        let prev_time = player.get_search_time();
                        player = MctsPlayer::default().set_search_time(prev_time - 100);
                    }
                    't' => {
                        println!("current search time: {}", player.get_search_time());
                    }
                    'c' => {
                        println!("\x1B[2J\x1B[1;1H");
                    }
                    'd' => {
                        dbg!(&state);
                    }
                    'p' => {
                        dbg!(state.possible_actions());
                    }
                    'n' => {
                        state = Round::new(romu::range_usize(0..4));
                    }
                    'a' => {
                        let action = player.decide(state);
                        println!("player {} plays {action:?}\n", state.turn());
                        state.apply_action(action);
                    }
                    _ => (),
                }
            }
        }
    }
}
