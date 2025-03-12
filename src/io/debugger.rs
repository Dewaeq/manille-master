use ismcts::state::State;

use crate::action::Action;
use crate::action_collection::ActionCollection;
use crate::inference::Inference;
use crate::io::input;
use crate::players::mcts_player::MctsPlayer;
use crate::players::Player;
use crate::round::{Round, RoundPhase};
use crate::stack::Stack;

pub fn run() {
    let mut state = Round::new(romu::range_usize(0..4));
    let mut inference = Inference::default();
    let mut player = MctsPlayer::new(1000, true);
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
                    dbg!(&inference);
                }
                'p' => {
                    dbg!(state.possible_actions());
                }
                'n' => {
                    state = Round::new(romu::range_usize(0..4));
                }
                'r' => {
                    state = state.randomize(state.turn(), &inference);
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
                        state = state.observe_action(observer.unwrap(), action, &inference);
                    }
                }
                'l' => {
                    let cards = state.player_cards(0);
                    println!("{cards:?}");
                }
                'm' => {
                    let action = request_action(state.possible_actions()).pop().unwrap();
                    inference.infer(&state, action, state.turn());
                    state.apply_action(action);
                }
                'a' => {
                    let action = player.decide(state, &inference);
                    println!("player {} plays {action:?}\n", state.turn());
                    inference.infer(&state, action, state.turn());
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
