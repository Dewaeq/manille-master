use ismcts::{action_list::ActionList, searcher::Searcher, state::State};

use super::Player;
use crate::{action::Action, inference::Inference, round::Round};

pub struct MctsPlayer {
    searcher: Searcher<Round>,
    search_time: u128,
    use_inference: bool,
}

impl Player for MctsPlayer {
    fn decide(&mut self, round: Round, inference: &Inference) -> Action {
        #[cfg(not(feature = "debug"))]
        {
            let mut actions = round.possible_actions();
            if actions.len() == 1 {
                return actions.pop_random().unwrap();
            }
        }
        let inference = if self.use_inference {
            inference
        } else {
            &Inference::default()
        };

        let result = self.searcher.search(&round, inference, self.search_time);
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

impl MctsPlayer {
    pub fn new(search_time: u128, use_inference: bool) -> Self {
        MctsPlayer {
            searcher: Searcher::default(),
            search_time,
            use_inference,
        }
    }

    pub fn set_search_time(mut self, time: u128) -> Self {
        self.search_time = time;
        self
    }

    pub const fn get_search_time(&self) -> u128 {
        self.search_time
    }
}

impl Default for MctsPlayer {
    fn default() -> Self {
        MctsPlayer {
            searcher: Searcher::default(),
            search_time: 500,
            use_inference: true,
        }
    }
}
