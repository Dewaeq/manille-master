use eframe::egui;
use ismcts::{action_list::ActionList, state::State};
use log::info;

use crate::{
    action::Action,
    card::Card,
    io::card_image_src,
    players::{mcts_player::MctsPlayer, Player},
    round::{Round, RoundPhase},
};

#[derive(Default)]
enum Screen {
    #[default]
    Home,
    Game,
}

pub struct App {
    screen: Screen,
    card_history: Vec<Card>,
    action_history: Vec<(usize, Action)>,
    round: Round,
    num_rounds: usize,
    round_is_finished: bool,
    scores: [i16; 2],
    ai_player: MctsPlayer,
}

impl Default for App {
    fn default() -> Self {
        App {
            screen: Screen::Home,
            action_history: vec![],
            card_history: vec![],
            round: Round::new(0),
            num_rounds: 0,
            round_is_finished: false,
            ai_player: MctsPlayer::default().set_search_time(150),
            scores: [0; 2],
        }
    }
}

impl App {
    fn apply_action(&mut self, action: Action) {
        if let Action::PlayCard(card) = action {
            self.card_history.push(card);
        }
        self.action_history.push((self.round.turn(), action));
        self.round.apply_action(action);
    }

    fn click_action(&mut self, action: Action, ctx: &eframe::egui::Context) {
        info!("clicked on {action:?}");
        if self.round.turn() != 0 || !self.round.possible_actions().has(&action) {
            return;
        }

        self.apply_action(action);

        self.do_ai_moves();

        if self.round.is_terminal() {
            self.round_is_finished = true;
        }
    }

    fn finish_round(&mut self) {
        if !self.round_is_finished {
            return;
        }

        let scores = self.round.scores();
        let winning_team = if scores[0] > scores[1] { 0 } else { 1 };
        info!("round scores: {scores:?}");
        self.scores[winning_team] += scores[winning_team] - 30;

        assert!(scores.iter().sum::<i16>() == 60);
        self.num_rounds += 1;
        self.round.setup_for_next_round();
        self.card_history.clear();
        self.action_history.clear();
        self.round_is_finished = false;

        self.do_ai_moves();
    }

    fn do_ai_moves(&mut self) {
        while !self.round.is_terminal() && self.round.turn() != 0 {
            let action = self.ai_player.decide(self.round);
            self.apply_action(action);
        }
    }
}

impl App {
    pub fn name() -> &'static str {
        "Manille Master"
    }

    fn display_home(&mut self, ctx: &eframe::egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Manille Master");
                if ui.button("New Game").clicked() {
                    self.screen = Screen::Game;
                }
            });
        });
    }

    fn display_game(&mut self, ctx: &eframe::egui::Context) {
        let screen_width = ctx.screen_rect().width();
        let card_width = screen_width * 0.6 / 8.;

        egui::TopBottomPanel::top("scores_and_trump").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("scores: {:?}", self.scores));
                    ui.label(format!("trump: {:?}", self.round.trump()));
                });
                ui.label(format!("round score: {:?}", self.round.scores()));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cards = self.round.trick_ref().cards().into_vec();

            if self.round_is_finished && ui.button("Start next round").clicked() {
                self.finish_round();
            }

            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    self.show_cards(card_width * 1.6, &cards, ui);
                    if self.card_history.len() >= 4 {
                        let in_this_round = self.round.trick_ref().cards().len();
                        let end = self.card_history.len() - in_this_round;
                        let cards = self.card_history[(end - 4)..end].to_vec();

                        self.show_cards(card_width * 0.5, &cards, ui);
                    }
                });
                ui.vertical(|ui| {
                    for &(player, action) in &self.action_history {
                        ui.label(format!("player {player} plays {action}"));
                    }
                })
            });
        });

        egui::TopBottomPanel::bottom("player_cards").show(ctx, |ui| {
            ui.vertical_centered(|ui| match self.round.phase() {
                RoundPhase::PickTrump if self.round.turn() == 0 => {
                    self.show_trump_actions(ui);
                    let cards = self.round.player_cards(0).into_vec();
                    self.show_cards(card_width, &cards, ui);
                }
                _ => {
                    let cards = self.round.player_cards(0).into_vec();
                    self.show_cards(card_width, &cards, ui);
                }
            });
        });
    }

    fn show_trump_actions(&mut self, ui: &mut egui::Ui) {
        let actions = self.round.possible_actions().to_vec();
        ui.horizontal(|ui| {
            for action in actions {
                if ui.button(format!("{action:?}")).clicked() {
                    self.click_action(action, ui.ctx());
                }
            }
        });
    }

    fn show_cards(&mut self, card_width: f32, cards: &Vec<Card>, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for &card in cards {
                let src = card_image_src(&card);
                let btn = egui::ImageButton::new(src);

                let enabled = self.round.turn() == 0
                    && self.round.phase() == RoundPhase::PlayCards
                    && self.round.possible_actions().has(&Action::PlayCard(card));

                ui.add_enabled_ui(enabled, |ui| {
                    if ui.add_sized([card_width, card_width * 1.5], btn).clicked() {
                        self.click_action(Action::PlayCard(card), ui.ctx());
                    }
                });
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Home => self.display_home(ctx),
            Screen::Game => self.display_game(ctx),
        }
    }
}
