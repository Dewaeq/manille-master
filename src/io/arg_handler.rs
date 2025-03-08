use crate::bench::bench;
use crate::players::{mcts_player::MctsPlayer, random_player::RandomPlayer, Player, PlayerVec};
use crate::sprt::run_sprt;
use crate::tournament::run_tournament_multithreaded;

use super::{debugger, input};

pub fn handle_args(args: Vec<String>) {
    if args.contains(&"bench".to_owned()) {
        let size = input::read_parsed("number of games: ").ok();
        bench(size);
    }

    if args.contains(&"d".to_owned()) {
        debugger::run();
    }

    if args.contains(&"sprt".to_owned()) {
        let think_time = input::read_parsed("think time: ").unwrap_or(100);
        let player_gen = move || -> PlayerVec {
            vec![
                RandomPlayer::boxed(),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                RandomPlayer::boxed(),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
            ]
        };
        run_sprt(14, player_gen);
    }

    if args.contains(&"tournament".to_owned()) {
        let num_games = input::read_parsed("games per thread: ").unwrap_or(5);
        let num_threads = input::read_parsed("threads: ").unwrap_or(14);
        let think_time = input::read_parsed("think time: ").unwrap_or(100);

        let player_gen = move || -> PlayerVec {
            vec![
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                RandomPlayer::boxed(),
                Box::new(MctsPlayer::default().set_search_time(think_time)),
                RandomPlayer::boxed(),
            ]
        };

        run_tournament_multithreaded(num_games, num_threads, "mcts vs random", false, player_gen);
    }
}
