use ismcts::state::State;

use crate::io::input;
use crate::players::{mcts_player::MctsPlayer, Player};
use crate::round::Round;

pub fn run() {
    let mut state = Round::new(romu::range_usize(0..4));
    let mut player = MctsPlayer::default().set_search_time(1_000);

    loop {
        for c in input::read_line().chars() {
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
                'i' => {
                    state = input::read_round();
                }
                'm' => {
                    let possible_actions = state.possible_actions().to_vec();
                    for (i, a) in possible_actions.iter().enumerate() {
                        println!("{i}: {a:?}");
                    }
                    loop {
                        if let Ok(idx) = input::read_parsed::<usize>("action index:") {
                            if idx < possible_actions.len() {
                                let action = possible_actions[idx];
                                state.apply_action(action);
                                break;
                            }
                        }
                    }
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
