use std::io::stdin;

use crate::mcts::{action_list::ActionList, state::State};

use super::Player;

#[derive(Default)]
pub struct HumanPlayer;

impl Player for HumanPlayer {
    fn decide(&mut self, round: crate::round::Round) -> crate::action::Action {
        let my_cards = round.player_cards(round.turn());
        println!("your cards: {my_cards:?}");
        let possible_actions = round.possible_actions();
        println!("possible actions:\n{possible_actions}");

        let mut buf = String::new();

        loop {
            buf.clear();
            stdin().read_line(&mut buf).unwrap();

            if let Ok(index) = buf.trim().parse::<usize>() {
                if index < possible_actions.len() {
                    let action = possible_actions.to_vec()[index];
                    println!("you played {action:?}");
                    return action;
                }
            }
        }
    }
}
