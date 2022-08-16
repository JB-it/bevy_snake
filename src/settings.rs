use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use crate::{game_data::GameData, game_state::GameState};

pub struct Settings {
    pub wraparound: bool,
    pub foob_color: Color32,
    pub snek_head_color: Color32,
    pub snek_body_color: Color32,
    pub background: Color32,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            wraparound: true,
            foob_color: Color32::RED,
            snek_head_color: Color32::GREEN,
            snek_body_color: Color32::DARK_GREEN,
            background: Color32::DARK_GRAY,
        }
    }
}

pub fn settings_ui(
    mut egui_context: ResMut<EguiContext>,
    game_data: Res<GameData>,
    mut settings: ResMut<Settings>,
) {
    if game_data.game_state != GameState::Menu {
        return;
    }

    egui::Window::new("Settings").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Looks");
        ui.horizontal(|ui| {
            ui.label("Food color");
            ui.color_edit_button_srgba(&mut settings.foob_color);
        });
        ui.horizontal(|ui| {
            ui.label("Sneak Head color");
            ui.color_edit_button_srgba(&mut settings.snek_head_color);
        });
        ui.horizontal(|ui| {
            ui.label("Sneak Body color");
            ui.color_edit_button_srgba(&mut settings.snek_body_color);
        });
        ui.horizontal(|ui| {
            ui.label("Background");
            ui.color_edit_button_srgba(&mut settings.background);
        });
        ui.heading("Gameplay");
        ui.checkbox(&mut settings.wraparound, "Wrap around");
    });
}
