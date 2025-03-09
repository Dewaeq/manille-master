use eframe::egui;
use ismcts::{action_list::ActionList, state::State};
use log::info;

use crate::{
    action::Action,
    card::Card,
    io::card_image_src,
    round::{Round, RoundPhase},
};

enum Screen {
    Home,
    Game,
}

pub struct App {
    screen: Screen,
    round: Round,
    num_rounds: usize,
    scores: [i32; 2],
}

impl Default for App {
    fn default() -> Self {
        App {
            screen: Screen::Home,
            round: Round::new(0),
            num_rounds: 0,
            scores: [0; 2],
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let played_cards = self.round.trick_ref().cards();
                for card in played_cards.iter() {
                    let src = card_image_src(card);
                    let img = egui::Image::new(src)
                        .fit_to_exact_size(egui::Vec2::new(card_width, card_width * 1.5));
                    ui.add(img);
                }
            });
        });

        egui::TopBottomPanel::bottom("player_cards").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let human_cards = self.round.player_cards(0);
                    for card in human_cards.into_iter() {
                        let src = card_image_src(&card);
                        let img = egui::Image::new(src)
                            .fit_to_exact_size(egui::Vec2::new(card_width, card_width * 1.5));
                        if ui.add(img).clicked() {
                            info!("selected {card}");
                            self.select_card(card);
                        }
                    }
                });
            });
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
