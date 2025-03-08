use std::{fmt::Debug, io::stdin, str::FromStr};

use crate::{
    round::{Round, RoundPhase},
    stack::Stack,
    suit::Suit,
};

pub fn read_round() -> Round {
    let observer_cards = read_stack("observer cards: ");
    println!("{observer_cards:?}");
    let played_cards = read_stack("played cards: ");
    println!("{played_cards:?}");
    let player_card_counts = read_vec_parsed("card counts: ").try_into().unwrap();
    let dealer = read_parsed("dealer: ").unwrap();
    let turn = read_parsed("turn: ").unwrap();
    let scores = read_vec_parsed("scores: ").try_into().unwrap();
    let phase = unsafe {
        let idx = read_parsed::<u8>("phase: ").unwrap();
        std::mem::transmute::<u8, RoundPhase>(idx)
    };
    let trump = {
        let idx = read_parsed::<u8>("trump: ").unwrap();
        if idx == 4 {
            None
        } else {
            Some(Suit::from(idx))
        }
    };

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

fn read_vec_parsed<T: FromStr>(message: &str) -> Vec<T>
where
    T::Err: Debug,
{
    println!("{message}");

    read_line()
        .split_whitespace()
        .map(|c| c.parse::<T>().unwrap())
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
