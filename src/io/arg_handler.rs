use crate::players::mcts_player::MctsPlayer;
use crate::players::random_player::RandomPlayer;
use crate::players::PlayerVec;
use crate::sprt::run_sprt;
use crate::tournament::run_tournament_multithreaded;
use crate::{bench::bench, players::Player};

use super::debugger::Debugger;
use super::input;

pub fn handle_args(args: Vec<String>) {
    if args.contains(&"bench".to_owned()) {
        let size = input::read_parsed("number of games: ").ok();
        bench(size);
    }

    if args.contains(&"d".to_owned()) {
        let mut d = Debugger::new();
        d.run();
    }

    if args.contains(&"sprt".to_owned()) {
        let think_time = input::read_parsed("think time: ").unwrap_or(100);
        let player_gen = move || -> PlayerVec {
            vec![
                Box::new(MctsPlayer::new(think_time, true)),
                Box::new(MctsPlayer::new(think_time, false)),
                Box::new(MctsPlayer::new(think_time, true)),
                Box::new(MctsPlayer::new(think_time, false)),
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
                RandomPlayer::boxed(),
                Box::new(MctsPlayer::new(think_time, true)),
                RandomPlayer::boxed(),
                Box::new(MctsPlayer::new(think_time, true)),
            ]
        };

        run_tournament_multithreaded(num_games, num_threads, "mcts vs random", false, player_gen);
    }
}
