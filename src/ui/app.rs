use eframe::egui;
use ismcts::{action_list::ActionList, state::State};
use log::info;

use crate::{
    action::Action,
    card::Card,
    io::card_image_src,
    players::{mcts_player::MctsPlayer, Player},
    round::{Round, RoundPhase},
    trick::Trick,
};

enum Screen {
    Home,
    Game,
}

pub struct App {
    screen: Screen,
    round: Round,
    prev_trick: Option<Trick>,
    num_rounds: usize,
    scores: [i32; 2],
    ai_player: MctsPlayer,
}

impl Default for App {
    fn default() -> Self {
        App {
            screen: Screen::Home,
            round: Round::new(0),
            num_rounds: 0,
            scores: [0; 2],
            ai_player: MctsPlayer::default(),
            prev_trick: None,
        }
    }
}

impl App {
    fn select_card(&mut self, card: Card) {
        if self.round.turn() == 0 {
            let possible_actions = self.round.possible_actions();
            let action = match self.round.phase() {
                RoundPhase::PlayCards => Action::PlayCard(card),
                RoundPhase::PickTrump => Action::PickTrump(Some(card.suite())),
            };

            if possible_actions.has(&action) {
                self.round.apply_action(action);

                while !self.round.is_terminal() && self.round.turn() != 0 {
                    let action = self.ai_player.decide(self.round);
                    self.round.apply_action(action);
                }
            }
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
        let card_width = screen_width * 0.4 / 8.;

        egui::TopBottomPanel::top("scores_and_trump").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!("scores: {:?}", self.scores));
                ui.label(format!("trump: {:?}", self.round.trump()));
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cards = self
                .round
                .trick_ref()
                .cards()
                .iter()
                .copied()
                .collect::<Vec<_>>();

            ui.vertical_centered(|ui| {
                self.show_cards(card_width, &cards, ui);
            });
        });

        egui::TopBottomPanel::bottom("player_cards").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let cards = self.round.player_cards(0).into_iter().collect::<Vec<_>>();
                self.show_cards(card_width, &cards, ui);
            });
        });
    }

    fn show_cards(&mut self, card_width: f32, cards: &Vec<Card>, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for &card in cards {
                let src = card_image_src(&card);
                let btn = egui::ImageButton::new(src);

                let enabled = self.round.turn() == 0
                    && (self.round.phase() == RoundPhase::PickTrump
                        || self.round.possible_actions().has(&Action::PlayCard(card)));

                ui.add_enabled_ui(enabled, |ui| {
                    if ui.add_sized([card_width, card_width * 1.5], btn).clicked() {
                        self.select_card(card);
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
