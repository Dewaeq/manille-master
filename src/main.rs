use std::io::stdin;

use bench::bench;
use game_state::GameState;
use mcts::state::State;
use players::{mcts_player::MctsPlayer, random_player::RandomPlayer, Player, PlayerVec};
use sprt::run_sprt;

mod action;
mod action_collection;
mod array;
mod bench;
mod bits;
mod card;
mod game;
mod game_phase;
mod game_state;
mod mcts;
mod players;
mod sprt;
mod stack;
mod suite;
mod trick;

fn main() {
    romu::seed();

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"sprt".to_owned()) {
        let think_time = args
            .last()
            .and_then(|x| x.parse::<u128>().ok())
            .unwrap_or(10);

        let player_gen = move || -> PlayerVec {
            vec![
                Box::new(RandomPlayer::default()),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                Box::new(RandomPlayer::default()),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
            ]
        };
        run_sprt(14, player_gen);
    }

    if args.contains(&"bench".to_owned()) {
        let size = args.last().and_then(|x| x.parse::<usize>().ok());
        bench(size);
    }

    if args.contains(&"d".to_owned()) {
        let mut state = GameState::new();
        let mut player = MctsPlayer::default().set_search_time(2000);
        //player.set_index(state.turn());

        let mut buf = String::new();

        loop {
            buf.clear();
            stdin().read_line(&mut buf).unwrap();

            match buf.trim() {
                "q" => break,
                "c" => {
                    println!("\x1B[2J\x1B[1;1H");
                    //println!("hi");
                    //println!("hi");
                    //println!("hi");
                    //println!("hi");
                }
                "d" => {
                    dbg!(&state);
                }
                "p" => {
                    dbg!(state.possible_actions());
                }
                "n" => {
                    state = GameState::new();
                    player.set_index(state.turn());
                }
                "a" => {
                    let action = player.decide(state);
                    println!("player {} plays {action:?}\n", state.turn());
                    state.apply_action(action);
                }
                _ => (),
            }
        }
    }
}
