use std::{io::stdin, str::FromStr};

use crate::{
    round::{Round, RoundPhase},
    stack::Stack,
    suit::Suit,
};

pub fn read_round() -> Round {
    let observer_cards = read_stack("observer cards: ");

    println!("{observer_cards:?}");

    let phase = unsafe {
        let idx = read_parsed::<u8>("phase (0): ").unwrap_or(0);
        std::mem::transmute::<u8, RoundPhase>(idx)
    };

    let mut dealer = 0;
    let mut turn = 1;
    let mut played_cards = Stack::default();
    let mut player_card_counts = [8; 4];
    let mut trump = None;
    let mut scores = [0; 2];

    if phase == RoundPhase::PickTrump {
        dealer = read_parsed("dealer (0): ").unwrap_or(dealer);
        turn = (dealer + 1) % 4;
    }
    if phase == RoundPhase::PlayCards {
        played_cards = read_stack("played cards: ");
        println!("{played_cards:?}");
        player_card_counts = read_vec_parsed("card counts ([8, 8, 8, 8]): ")
            .try_into()
            .unwrap_or(player_card_counts);

        let trump_idx = read_parsed::<u8>("trump: ").unwrap();
        trump = if trump_idx == 4 {
            None
        } else {
            Some(Suit::from(trump_idx))
        };

        scores = read_vec_parsed("scores ([0, 0]): ")
            .try_into()
            .unwrap_or(scores);

        turn = read_parsed("turn (1): ").unwrap_or(turn);
    }

    Round::from_observer(
        observer_cards,
        played_cards,
        player_card_counts,
        dealer,
        turn,
        phase,
        trump,
        scores,
    )
}

pub fn read_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

pub fn read_parsed<T: FromStr>(message: &str) -> Result<T, T::Err> {
    println!("{message}");

    read_line().trim().parse::<T>()
}

pub fn read_vec_parsed<T: FromStr>(message: &str) -> Vec<T> {
    println!("{message}");

    read_line()
        .split_whitespace()
        .flat_map(|c| c.parse::<T>())
        .collect::<Vec<_>>()
}

fn read_stack(message: &str) -> Stack {
    println!("{message}");
    let all = Stack::ALL.into_iter().collect::<Vec<_>>();
    for i in 0..8 {
        print!("{}: {}\t", i, all[i]);
        print!("{}: {}\t", i + 8, all[i + 8]);
        print!("{}: {}\t", i + 16, all[i + 16]);
        println!("{}: {}\t", i + 24, all[i + 24]);
    }

    let mut stack = Stack::default();

    for part in read_line().split_whitespace() {
        if let Ok(idx) = part.parse::<u32>() {
            if idx < Stack::ALL.len() {
                stack |= 1 << idx;
            }
        }
    }

    stack
}
