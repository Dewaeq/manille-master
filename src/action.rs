use crate::{card::Card, suite::Suite};

pub enum Action {
    Trick(Suite),
    Card(Card),
}
