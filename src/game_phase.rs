#[derive(Default, Clone, Debug)]
pub enum GamePhase {
    #[default]
    PickingTrump,
    PlayingRound,
    Finished {
        winning_team: usize,
    },
}
