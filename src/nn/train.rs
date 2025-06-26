use jabba::{
    activation::ActivationType,
    nn::{NNBuilder, NNOptions},
    optimizers::OptimizerType,
    storage::write_to,
    Matrix,
};

use crate::{action::Action, round::RoundPhase};

use super::logger::read_log;

pub fn train(data_path: &str) {
    let batch_size = 128;
    let options = NNOptions {
        log_interval: Some(1),
        batch_size,
        test: true,
        ..Default::default()
    };
    let num_inputs = 32 /* players cards*/
        + 32 /*played cards*/
        + 32 /*cards in current trick*/
        + 5 /*trump (4 suits + none)*/
        + 2 /* phase (pick trump or play card)*/;
    let num_outputs = 32 /* card to play*/
        + 5 /* trump to pick*/;
    let mut model = NNBuilder::new(num_inputs)
        .options(options)
        .add_layer(150, ActivationType::ReLuLeaky)
        .add_layer(150, ActivationType::ReLuLeaky)
        .add_layer(150, ActivationType::ReLuLeaky)
        .add_layer(num_outputs, ActivationType::Sigmoid)
        .optimizer(OptimizerType::Adam)
        .build();

    let states = read_log(data_path);
    let num_states = states.len();
    println!("read {num_states} states from archive");
    let mut x = Matrix::zeros(num_inputs, num_states);
    let mut y = Matrix::zeros(num_outputs, num_states);

    let mut i = 0;

    for (entry, (mut x_col, mut y_col)) in states
        .iter()
        .zip(x.column_iter_mut().zip(y.column_iter_mut()))
    {
        for card in entry.state.player_cards(entry.actor).into_iter() {
            x_col[card.get_index() as usize] = 1.;
        }
        for card in entry.state.played_cards().into_iter() {
            x_col[32 + card.get_index() as usize] = 1.;
        }
        for card in entry.state.trick_ref().cards().iter() {
            x_col[2 * 32 + card.get_index() as usize] = 1.;
        }
        if entry.state.phase() == RoundPhase::PickTrump {
            let index = entry.state.trump().map_or(4, |suit| suit as usize);
            x_col[3 * 32 + index] = 1.;
            x_col[3 * 32 + 5] = 1.;
        } else {
            x_col[3 * 32 + 5 + 1] = 1.;
        }
        match entry.action {
            Action::PlayCard(card) => {
                y_col[card.get_index() as usize] = 1.;
            }
            Action::PickTrump(trump) => {
                let index = trump.map_or(4, |suit| suit as usize);
                y_col[32 + index] = 1.;
            }
        }

        i += 1;
        if i % 10 == 0 {
            println!("\x1B[2J\x1B[1;1H");
            println!("progress: {}%", (i as f32) / (num_states as f32) * 100.);
        }
    }

    println!("parsed all data!");

    let top = num_states * 8 / 10 - (num_states * 8 / 10) % batch_size;
    let r1 = 0..top;
    let r2 = top..num_states;
    let (x_train, x_test) = x.columns_range_pair(r1.clone(), r2.clone());
    let (y_train, y_test) = y.columns_range_pair(r1, r2);

    model.train(
        &x_train.clone_owned(),
        &y_train.clone_owned(),
        &x_test.clone_owned(),
        &y_test.clone_owned(),
    );

    write_to("models/test.model", &model).unwrap();
}
