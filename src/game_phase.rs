#[derive(Default, Clone, Copy, Debug)]
pub enum GamePhase {
    #[default]
    PickingTrump,
    PlayingRound,
    Finished {
        winning_team: usize,
    },
}
