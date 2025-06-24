use std::collections::HashMap;

use ismcts::state::State;

use crate::action::Action;
use crate::action_collection::ActionCollection;
use crate::inference::Inference;
use crate::io::input;
use crate::players::mcts_player::MctsPlayer;
use crate::players::Player;
use crate::round::{Round, RoundPhase};
use crate::stack::Stack;

struct Command {
    name: char,
    description: String,
    task: fn(&mut Debugger),
}

pub struct Debugger {
    commands: HashMap<char, Command>,
    state: Round,
    inference: Inference,
    player: MctsPlayer,
}

impl Debugger {
    pub fn new() -> Self {
        let mut debugger = Debugger {
            commands: Default::default(),
            state: Round::new(romu::range_usize(0..4)),
            inference: Default::default(),
            player: MctsPlayer::new(1000, true),
        };
        debugger.add_command(Command {
            name: '+',
            description: "increase search time by 100ms".to_owned(),
            task: |d| {
                let prev_time = d.player.get_search_time();
                d.player.set_search_time(prev_time + 100);
            },
        });
        debugger.add_command(Command {
            name: '-',
            description: "decrease search time by 100ms".to_owned(),
            task: |d| {
                let prev_time = d.player.get_search_time();
                d.player.set_search_time(prev_time - 100);
            },
        });
        debugger.add_command(Command {
            name: 't',
            description: "print current search time".to_owned(),
            task: |d| {
                let time = d.player.get_search_time();
                println!("current search time: {time}");
            },
        });
        debugger.add_command(Command {
            name: 'c',
            description: "clear screen".to_owned(),
            task: |_| {
                println!("\x1B[2J\x1B[1;1H");
            },
        });
        debugger.add_command(Command {
            name: 'd',
            description: "print round state and inference".to_owned(),
            task: |d| {
                dbg!(&d.state);
                dbg!(&d.inference);
            },
        });
        debugger.add_command(Command {
            name: 'p',
            description: "print possible actions".to_owned(),
            task: |d| {
                dbg!(d.state.possible_actions());
            },
        });
        debugger.add_command(Command {
            name: 'n',
            description: "start new round".to_owned(),
            task: |d| {
                d.state = Round::new(romu::range_usize(0..4));
                d.inference = Inference::default();
            },
        });
        debugger.add_command(Command {
            name: 'r',
            description: "randomize current round, except current player".to_owned(),
            task: |d| d.state = d.state.randomize(d.state.turn(), &d.inference),
        });
        debugger.add_command(Command {
            name: 'i',
            description: "read an entire state from stdin".to_owned(),
            task: |d| {
                d.state = input::read_round();
                d.inference = Default::default();
            },
        });
        debugger.add_command(Command {
            name: 'f',
            description: "todo".to_owned(),
            task: |d| {
                let actions = match d.state.phase() {
                    RoundPhase::PickTrump => ActionCollection::Trumps(0b11111),
                    RoundPhase::PlayCards => {
                        ActionCollection::Cards(Stack::ALL ^ d.state.played_cards())
                    }
                };
                let actions = request_action(actions);
                for action in actions {
                    // d.state = d
                    //     .state
                    //     .observe_action(observer.unwrap(), action, &d.inference);
                }
            },
        });
        debugger.add_command(Command {
            name: 'l',
            description: "list all cards of player 0".to_owned(),
            task: |d| {
                let cards = d.state.player_cards(0);
                println!("{cards:?}");
            },
        });
        debugger.add_command(Command {
            name: 'm',
            description: "manually select an action for the current player".to_owned(),
            task: |d| {
                let actions = request_action(d.state.possible_actions());
                if actions.len() != 1 {
                    println!("operation aborted! should select only one action");
                } else {
                    let action = actions[0];
                    d.inference.infer(&d.state, action, d.state.turn());
                    d.state.apply_action(action);
                }
            },
        });
        debugger.add_command(Command {
            name: 'a',
            description: "let mcts select the current player's action".to_owned(),
            task: |d| {
                let action = d.player.decide(d.state, &d.inference);
                println!("player {} plays {action:?}\n", d.state.turn());
                d.inference.infer(&d.state, action, d.state.turn());
                d.state.apply_action(action);
            },
        });

        debugger
    }

    pub fn run(&mut self) {
        loop {
            for c in input::read_line().chars() {
                match c {
                    'q' => return,
                    'h' => self.print_help(),
                    _ => {
                        if let Some(command) = self.commands.get(&c) {
                            (command.task)(self);
                        }
                    }
                }
            }
        }
    }

    fn add_command(&mut self, command: Command) {
        self.commands.insert(command.name, command);
    }

    fn print_help(&self) {
        println!("Manille master debug commands:");
        let mut commands = self.commands.values().collect::<Vec<_>>();
        commands.sort_by_key(|c| c.name);

        for command in commands {
            println!("{}:\t{}", command.name, command.description);
        }
        println!("q:\tquit");
        println!();
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

        println!("selected: {selected_actions:#?}, press y to confirm");
        if input::read_line().contains("y") {
            return selected_actions;
        }
    }
}
