use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use ismcts::{searcher::SearchResult, state::State};

use crate::{
    action::Action,
    inference::Inference,
    players::{mcts_player::MctsPlayer, Player},
    round::Round,
};

pub struct UiGame {
    pub round: Round,
    pub inference: Inference,
    pub num_rounds: usize,
    pub scores: [i16; 2],
    pub is_thinking: bool,
    pub think_time: f32,
    done_flag: Arc<AtomicBool>,
    result_slot: Arc<Mutex<Option<Action>>>,
    search_result_slot: Arc<Mutex<Option<SearchResult<Round>>>>,
}

impl Default for UiGame {
    fn default() -> Self {
        UiGame {
            round: Round::new(0),
            inference: Default::default(),
            num_rounds: Default::default(),
            scores: Default::default(),
            is_thinking: false,
            done_flag: Arc::new(AtomicBool::new(false)),
            result_slot: Arc::new(Mutex::new(None)),
            search_result_slot: Default::default(),
            think_time: 500.,
        }
    }
}

impl UiGame {
    pub fn apply_action(&mut self, action: Action) {
        self.inference.infer(&self.round, action, self.round.turn());
        self.round.apply_action(action);

        if self.round.is_terminal() {
            self.finish_round();
        }
    }

    pub fn start_thinking(&mut self) {
        self.is_thinking = true;
        let mut ai_player = MctsPlayer::new(self.think_time as _, true);
        let round = self.round;
        let inference = self.inference;

        let result_slot = Arc::clone(&self.result_slot);
        let search_result_slot = Arc::clone(&self.search_result_slot);
        let done_flag = Arc::clone(&self.done_flag);

        thread::spawn(move || {
            let action = ai_player.decide(round, &inference);
            println!("action: {action:?}");
            println!(
                "result exists: {}",
                ai_player.get_last_search_result().is_some()
            );
            *result_slot.lock().unwrap() = Some(action);
            *search_result_slot.lock().unwrap() = ai_player.get_last_search_result();
            done_flag.store(true, Ordering::Release);
        });
    }

    pub fn load_search_result(&self) -> Option<SearchResult<Round>> {
        if let Ok(s) = self.search_result_slot.lock() {
            return s.clone();
        }
        None
    }

    pub fn load_ai_move(&mut self) -> Option<Action> {
        if self.done_flag.load(Ordering::Acquire) {
            self.is_thinking = false;
            self.done_flag.store(false, Ordering::Release);
            if let Ok(mut result_slot) = self.result_slot.lock() {
                return Some(result_slot.take().unwrap());
            }
        }

        None
    }

    fn finish_round(&mut self) {
        let scores = self.round.scores();
        let winner = if scores[0] > scores[1] { 0 } else { 1 };
        self.scores[winner] += scores[winner] - 30;
        self.num_rounds += 1;
    }
}
