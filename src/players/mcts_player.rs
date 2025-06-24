use std::time::Duration;

use ismcts::{
    action_list::ActionList,
    searcher::{SearchResult, Searcher},
    state::State,
};

use super::Player;
use crate::{action::Action, inference::Inference, round::Round};

pub struct MctsPlayer {
    searcher: Searcher<Round>,
    search_time: u128,
    use_inference: bool,
    last_search_result: Option<SearchResult<Round>>,
}

impl Player for MctsPlayer {
    fn decide(&mut self, round: Round, inference: &Inference) -> Action {
        #[cfg(not(feature = "debug"))]
        {
            let mut actions = round.possible_actions();
            if actions.len() == 1 {
                self.last_search_result = Some(SearchResult {
                    num_simulations: 0,
                    duration: Duration::default(),
                    best_action: None,
                    child_stats: vec![],
                });
                return actions.pop_random().unwrap();
            }
        }
        let inference = if self.use_inference {
            inference
        } else {
            &Inference::default()
        };

        let result = self.searcher.search(&round, inference, self.search_time);
        self.last_search_result = Some(result.clone());
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
            last_search_result: Default::default(),
        }
    }

    pub fn set_search_time(&mut self, time: u128) {
        self.search_time = time;
    }

    pub const fn get_search_time(&self) -> u128 {
        self.search_time
    }

    pub fn get_last_search_result(&self) -> Option<SearchResult<Round>> {
        self.last_search_result.clone()
    }
}

impl Default for MctsPlayer {
    fn default() -> Self {
        MctsPlayer {
            searcher: Searcher::default(),
            search_time: 500,
            use_inference: true,
            last_search_result: Default::default(),
        }
    }
}
