use jabba::{
    activation::ActivationType,
    nn::{NNBuilder, NNOptions},
};

pub fn train(data_path: String) {
    let options = NNOptions {
        log_interval: Some(4),
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
        .build();
}
