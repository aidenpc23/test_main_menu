use std::sync::Arc;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin};
use egui::{FontData, FontDefinitions, FontFamily, FontId, Frame, Sense};

const UI_FONT: &[u8] = include_bytes!("../assets/fonts/Mat Saleh.ttf");
const TITLE_FONT: &[u8] = include_bytes!("../assets/fonts/FiraSans-Bold.ttf");

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GameState {
    #[default]
    MainMenu,
    Running,
}

#[derive(Resource, Default)]
struct StateResource {
    state: GameState,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.13, 0.15, 0.18)))
        .init_resource::<StateResource>()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            ..default()
        }))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(EguiContextPass, (ui_main_menu, ui_running))
        .add_systems(Startup, setup_fonts)
        .add_systems(Update, game_state_shortcuts)
        .run();
}

fn setup_fonts(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "ui_font".to_owned(),
        Arc::new(FontData::from_static(UI_FONT)),
    );
    fonts.families.insert(
        FontFamily::Name("ui_font".into()),
        vec!["ui_font".to_owned()],
    );

    fonts.font_data.insert(
        "title_font".to_owned(),
        Arc::new(FontData::from_static(TITLE_FONT)),
    );
    fonts.families.insert(
        FontFamily::Name("title_font".into()),
        vec!["title_font".to_owned()],
    );

    ctx.set_fonts(fonts);
}

fn ui_main_menu(
    mut contexts: EguiContexts,
    mut state: ResMut<StateResource>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if state.state != GameState::MainMenu {
        return;
    }

    let ctx = contexts.ctx_mut();

    let panel_frame = Frame::NONE.inner_margin(egui::Margin::symmetric(20, 16));

    egui::SidePanel::left("main_menu_panel")
        .min_width(340.0)
        .max_width(360.0)
        .resizable(false)
        .frame(panel_frame)
        .show(ctx, |ui| {
            let visuals = ui.visuals_mut();
            visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
            visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
            visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;

            visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
            visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
            visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

            visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(220, 220, 220);
            visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(120, 170, 255);
            visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(170, 200, 255);

            visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::WHITE;

            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("MY AWESOME GAME")
                            .font(FontId::new(36.0, FontFamily::Name("title_font".into())))
                            .color(egui::Color32::from_rgb(230, 230, 180))
                            .strong(),
                    )
                    .selectable(false),
                );
                ui.add_space(50.0);

                if ui
                    .add_sized(
                        [200.0, 48.0],
                        egui::Label::new(
                            egui::RichText::new("Start")
                                .font(FontId::new(24.0, FontFamily::Name("ui_font".into()))),
                        )
                        .sense(Sense::click())
                        .selectable(false),
                    )
                    .clicked()
                {
                    state.state = GameState::Running;
                }
                ui.add_space(12.0);

                if ui
                    .add_sized(
                        [200.0, 48.0],
                        egui::Label::new(
                            egui::RichText::new("Settings")
                                .font(FontId::new(24.0, FontFamily::Name("ui_font".into()))),
                        )
                        .sense(Sense::click())
                        .selectable(false),
                    )
                    .clicked()
                {
                    // TODO: bring up settings window/state
                }
                ui.add_space(12.0);

                if ui
                    .add_sized(
                        [200.0, 48.0],
                        egui::Label::new(
                            egui::RichText::new("Quit")
                                .font(FontId::new(24.0, FontFamily::Name("ui_font".into()))),
                        )
                        .sense(Sense::click())
                        .selectable(false),
                    )
                    .clicked()
                    || keyboard_input.just_pressed(KeyCode::Escape)
                {
                    std::process::exit(0);
                }
            });
        });
}

fn ui_running(mut contexts: EguiContexts, state: Res<StateResource>) {
    if state.state != GameState::Running {
        return;
    }

    let ctx = contexts.ctx_mut();

    let central_frame = Frame::NONE
        .fill(egui::Color32::from_rgba_premultiplied(10, 10, 12, 200))
        .corner_radius(egui::CornerRadius::same(8));

    egui::CentralPanel::default()
        .frame(central_frame)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(220.0);
                ui.label(
                    egui::RichText::new("PLAYING GAME!")
                        .size(64.0)
                        .color(egui::Color32::from_rgb(240, 240, 240))
                        .strong(),
                );
            });
        });
}

fn game_state_shortcuts(
    mut state: ResMut<StateResource>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if state.state == GameState::Running && keyboard_input.just_pressed(KeyCode::F1) {
        state.state = GameState::MainMenu;
    }
}
