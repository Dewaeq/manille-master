use ismcts::state::State;

use crate::action::Action;
use crate::action_collection::ActionCollection;
use crate::io::input;
use crate::players::{mcts_player::MctsPlayer, Player};
use crate::round::{Round, RoundPhase};
use crate::stack::Stack;

pub fn run() {
    let mut state = Round::new(romu::range_usize(0..4));
    let mut player = MctsPlayer::default().set_search_time(1_000);
    let mut observer = None;

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
                    observer = Some(0);
                }
                'f' => {
                    let actions = match state.phase() {
                        RoundPhase::PickTrump => ActionCollection::Trumps(0b11111),
                        RoundPhase::PlayCards => {
                            ActionCollection::Cards(Stack::ALL ^ state.played_cards())
                        }
                    };
                    let actions = request_action(actions);
                    for action in actions {
                        state = state.observe_action(observer.unwrap(), action);
                    }
                }
                'l' => {
                    let cards = state.player_cards(0);
                    println!("{cards:?}");
                }
                'm' => {
                    let action = request_action(state.possible_actions()).pop().unwrap();
                    state.apply_action(action);
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

fn request_action(possible_actions: ActionCollection) -> Vec<Action> {
    let possible_actions = possible_actions.to_vec();

    for (i, a) in possible_actions.iter().enumerate() {
        println!("{i}: {a:?}");
    }

    loop {
        let indices = input::read_vec_parsed::<usize>("actions: ");
        let selected_actions = indices
            .into_iter()
            .filter(|&i| i < possible_actions.len())
            .map(|i| possible_actions[i])
            .collect::<Vec<_>>();

        println!("selected: {selected_actions:#?}");
        if input::read_line().contains("y") {
            return selected_actions;
        }
    }
}
