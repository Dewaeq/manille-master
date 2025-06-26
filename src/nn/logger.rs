use crate::{
    action::Action,
    inference::Inference,
    players::{mcts_player::MctsPlayer, Player},
    round::Round,
};
use ismcts::state::State;
use std::{
    io::stdin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{channel, Receiver},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub struct LogEntry {
    pub action: Action,
    pub actor: usize,
    pub state: Round,
    pub score: f32,
}

pub fn start_log(num_threads: usize) -> Vec<LogEntry> {
    let stop = Arc::new(AtomicBool::new(false));
    let num_games = Arc::new(AtomicUsize::new(0));
    let shared_log = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let stop = Arc::clone(&stop);
        let num_games = Arc::clone(&num_games);
        let shared_log = Arc::clone(&shared_log);
        let handle = std::thread::spawn(|| {
            log_thread(stop, num_games, shared_log);
        });
        handles.push(handle);
    }

    let stdin_channel = spawn_stdin_channel();
    let started = Instant::now();
    loop {
        if let Ok(input) = stdin_channel.try_recv() {
            if input.contains("q") {
                break;
            }
        }
        if started.elapsed().as_secs() % 300 == 0 {
            let entries = shared_log.lock().unwrap().clone();
            write_log(&entries);
        }

        std::thread::sleep(Duration::from_millis(100));
        println!("\x1B[2J\x1B[1;1H");
        println!("press q to stop");
        println!("logged {} games", num_games.load(Ordering::Relaxed));
    }

    stop.store(true, Ordering::Release);
    for handle in handles {
        handle.join().unwrap();
    }

    let entries = Arc::into_inner(shared_log).unwrap().into_inner().unwrap();
    write_log(&entries);
    entries
}

fn write_log(entries: &Vec<LogEntry>) {
    let mut bytes = vec![];
    for entry in entries {
        unsafe {
            let ptr = (entry as *const LogEntry) as *const u8;
            let size = std::mem::size_of::<LogEntry>();
            let slice = core::slice::from_raw_parts(ptr, size);
            bytes.extend_from_slice(slice);
        }
    }

    let now = chrono::Local::now();
    let time = now.format("%Y-%m-%d_%H-%M-%S");
    let path = format!("logs/log-{}.bin", time);
    std::fs::write(path, bytes).unwrap();
}

fn log_thread(
    stop: Arc<AtomicBool>,
    num_games: Arc<AtomicUsize>,
    shared_log: Arc<Mutex<Vec<LogEntry>>>,
) {
    let mut ai_player = MctsPlayer::new(20, true);

    loop {
        let entries = log_game(&mut ai_player);
        shared_log.lock().unwrap().extend(entries);
        num_games.fetch_add(1, Ordering::Relaxed);

        if stop.load(Ordering::Acquire) {
            break;
        }
    }
}

fn log_game(ai_player: &mut MctsPlayer) -> Vec<LogEntry> {
    let mut inference = Inference::default();
    let mut state = Round::new(romu::range_usize(0..4));
    let mut entries = vec![];

    while !state.is_terminal() {
        let actor = state.turn();
        let action = ai_player.decide(state, &inference);
        let entry = LogEntry {
            action,
            actor,
            state,
            score: 0.,
        };
        entries.push(entry);
        inference.infer(&state, action, actor);
        state.apply_action(action);
    }

    for entry in &mut entries {
        entry.score = state.reward(entry.actor);
    }

    entries
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = channel();

    std::thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        println!("thread read {buffer}");
        match tx.send(buffer) {
            Ok(_) => {}
            Err(_) => break,
        }
    });

    rx
}
