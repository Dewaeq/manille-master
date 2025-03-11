use ismcts::{action_list::ActionList, searcher::Searcher, state::State};

use crate::{action::Action, inference::Inference, round::Round};

pub type PlayerVec = Vec<Player>;

#[derive(Clone)]
pub enum Player {
    RandomPlayer,
    MctsPlayer {
        searcher: Searcher<Round>,
        search_time: u128,
        use_inference: bool,
    },
}

impl Player {
    pub fn decide(&mut self, round: Round, inference: &Inference) -> Action {
        match self {
            Player::RandomPlayer => round.possible_actions().pop_random().unwrap(),
            Player::MctsPlayer {
                searcher,
                search_time,
                use_inference,
            } => {
                #[cfg(not(feature = "debug"))]
                {
                    let mut actions = round.possible_actions();
                    if actions.len() == 1 {
                        return actions.pop_random().unwrap();
                    }
                }
                let inference = if *use_inference {
                    inference
                } else {
                    &Inference::default()
                };

                let result = searcher.search(&round, inference, *search_time);
                #[cfg(feature = "debug")]
                {
                    println!(
                        "ran {} simulations at {} sims/sec",
                        result.num_simulations,
                        result.num_simulations as f32 / result.duration.as_secs_f32()
                    );
                    for &(stats, action) in result.child_stats.iter() {
                        println!(
                            "{action}:\tscore={:.5},\tsims={}",
                            stats.avg_score * 30.,
                            stats.num_sims
                        );
                    }
                }
                result.best_action.unwrap()
            }
        }
    }
}
