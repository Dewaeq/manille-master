use std::{collections::HashMap, sync::OnceLock};

use ismcts::state::State;
use macroquad::{
    color::{Color, DARKGRAY, WHITE},
    math::{vec2, Vec2},
    miniquad::window::screen_size,
    texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
    time::get_frame_time,
    ui::{hash, root_ui, widgets, Skin},
    window::{clear_background, next_frame, screen_width},
};

use super::{
    get_card_size,
    hand::{Hand, SPACING_FACTOR},
    ui_card::UiCard,
    ui_game::UiGame,
};
use crate::{
    action::Action, action_collection::ActionCollection, round::RoundPhase, stack::Stack,
    ui::card_texture,
};

pub static TEXTURES: OnceLock<HashMap<u32, Texture2D>> = OnceLock::new();

pub struct App {
    game: UiGame,
    moving_cards: Vec<UiCard>,
    returning_cards: Vec<UiCard>,
    time_since_last_action: f32,
    wait_time: f32,
}

impl App {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();
        for card in Stack::ALL.into_iter() {
            textures.insert(card.get_index(), card_texture(card).await);
        }
        for i in 1..4 {
            let texture = load_texture(&format!("assets/bots/bot{i}.png")).await;
            textures.insert(10000 + i, texture.unwrap());
        }
        TEXTURES.set(textures).unwrap();

        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(45)
            .build();
        let skin = Skin {
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);

        App {
            game: Default::default(),
            moving_cards: vec![],
            returning_cards: vec![],
            time_since_last_action: 0.,
            wait_time: 0.,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(DARKGRAY);
            self.wait_time -= get_frame_time();

            self.clear_old_moving_cards();
            self.check_next_ai_move();
            self.render_bot_icons();
            self.render_stats();
            self.render_cards();

            if self.game.round.phase() == RoundPhase::PickTrump && self.game.round.turn() == 0 {
                self.render_pick_trump_message();
            }
            if self.game.round.is_terminal() {
                self.render_next_round_message();
            }

            next_frame().await;
        }
    }

    fn render_next_round_message(&mut self) {
        let (width, height) = screen_size();
        if root_ui().button(vec2(width * 0.45, height * 0.65), "Next round") {
            self.game.round.setup_for_next_round();
            self.moving_cards.clear();
            self.returning_cards.clear();
        }
    }

    fn render_pick_trump_message(&mut self) {
        let (width, height) = screen_size();
        root_ui().label(vec2(width * 0.45, height * 0.5 - 70.), "Select trump");
        if root_ui().button(vec2(width * 0.45, height * 0.5), "Play without trump") {
            self.apply_action(Action::PickTrump(None));
        }
    }

    fn render_bot_icons(&self) {
        let width = screen_width();
        for i in 1..4 {
            let pos = self.get_player_position(i);
            let texture = &TEXTURES.get().unwrap()[&(10000 + i as u32)];
            draw_texture_ex(
                texture,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width * 0.08, width * 0.08)),
                    ..Default::default()
                },
            );
        }
    }

    fn render_cards(&mut self) {
        let cards = self.game.round.player_cards(0);
        let legal_actions = self.game.round.possible_actions();
        let legal_cards = match legal_actions {
            ActionCollection::Cards(stack) => stack,
            ActionCollection::Trumps(_) => cards,
            _ => unreachable!(),
        };

        if let Some(ui_card) = Hand::draw(cards, legal_cards) {
            match self.game.round.phase() {
                RoundPhase::PickTrump => {
                    let trump = Some(ui_card.card.suit());
                    self.apply_action(Action::PickTrump(trump));
                }
                RoundPhase::PlayCards => {
                    self.apply_action(Action::PlayCard(ui_card.card));
                    self.play_card(ui_card);
                }
            }
        }

        for moving_card in self
            .returning_cards
            .iter_mut()
            .chain(self.moving_cards.iter_mut())
        {
            moving_card.draw();
            moving_card.update();
        }
    }

    fn render_stats(&mut self) {
        let (width, height) = screen_size();
        widgets::Group::new(hash!(), vec2(width * 0.3, height * 0.3)).ui(&mut root_ui(), |ui| {
            ui.label(None, &format!("Trump: {:?}", self.game.round.trump()));
            let scores = self.game.scores;
            let round_scores = self.game.round.scores();
            ui.label(None, &format!("Score: {} vs {}", scores[0], scores[1]));
            ui.label(
                None,
                &format!("Round score: {} vs {}", round_scores[0], round_scores[1]),
            );
            if let Some(s) = self.game.load_search_result() {
                ui.label(None, &format!("Ran: {} simulations", s.num_simulations));
                ui.label(
                    None,
                    &format!(
                        "at: {} sims/sec",
                        s.num_simulations as f32 / s.duration.as_secs_f32()
                    ),
                );
            }
        });
        widgets::Group::new(hash!(), vec2(width * 0.3, height * 0.3))
            .position(vec2(width * 0.7, 0.))
            .ui(&mut root_ui(), |ui| {
                ui.slider(
                    hash!(),
                    "Think time (ms)",
                    10f32..5000f32,
                    &mut self.game.think_time,
                );
            });
    }

    fn apply_action(&mut self, action: Action) {
        self.game.apply_action(action);
        self.time_since_last_action = 0.;
    }

    fn play_card(&mut self, mut ui_card: UiCard) {
        self.returning_cards.clear();
        if self.moving_cards.len() == 4 {
            self.moving_cards.clear();
        }

        ui_card.is_button = false;
        ui_card.is_moving = true;
        ui_card.target_pos = Some(self.get_card_target_pos());
        self.moving_cards.push(ui_card);
        if self.moving_cards.len() == 4 {
            self.wait_time = 1.7;
        }
    }

    fn clear_old_moving_cards(&mut self) {
        if self.moving_cards.len() == 4 && self.wait_time <= 0. {
            self.returning_cards.clear();
            let winner = self.game.round.turn();
            let pos = self.get_player_position(winner);
            for card in &mut self.moving_cards {
                card.target_pos = Some(pos);
                card.is_moving = true;
                self.returning_cards.push(card.clone());
            }
            self.moving_cards.clear();
            self.wait_time = 1.5;
        }

        if !self.returning_cards.is_empty() && self.returning_cards.iter().all(|c| !c.is_moving) {
            self.returning_cards.clear();
        }
    }

    fn get_card_target_pos(&self) -> Vec2 {
        let i = (self.game.round.played_cards().len() + 3) % 4;
        let (width, height) = screen_size();
        let card_width = get_card_size().x;
        let padding = width * 0.5 - card_width * SPACING_FACTOR * 2.;
        vec2(
            padding + card_width * SPACING_FACTOR * (i) as f32,
            height * 0.4,
        )
    }

    fn get_player_position(&self, player: usize) -> Vec2 {
        let (width, height) = screen_size();
        let positions = [
            vec2(0.45 * width, 0.95 * height),
            vec2(0.02 * width, 0.45 * height),
            vec2(0.45 * width, 0.02 * height),
            vec2(0.90 * width - 50., 0.45 * height),
        ];

        positions[player]
    }

    fn check_next_ai_move(&mut self) {
        let turn = self.game.round.turn();
        self.time_since_last_action += get_frame_time();

        if self.time_since_last_action > 1.5
            && self.wait_time <= 0.
            && turn != 0
            && !self.game.round.is_terminal()
            && !self.game.is_thinking
        {
            self.game.start_thinking();
        }

        if self.game.is_thinking {
            if let Some(action) = self.game.load_ai_move() {
                self.apply_action(action);
                match action {
                    Action::PlayCard(card) => {
                        let ui_card = UiCard::new(card, self.get_player_position(turn), false);
                        self.play_card(ui_card);
                    }
                    Action::PickTrump(trump) => {
                        println!("bot {turn} picked {trump:?}");
                    }
                }
            }
        }
    }
}
