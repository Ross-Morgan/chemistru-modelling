#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chemistru_modelling::simulator::Scenario;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

fn main() {
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    });

    App::new()
        .add_plugins(plugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, my_ui)
        .run();
}

fn my_ui(mut contexts: EguiContexts) {
    egui::Window::new("Control Panel").show(contexts.ctx_mut(), |ui| {
        ui.label("Control Panel Entry 1");
        ui.label("Control Panel Entry 2");

        let mut selected = Scenario::Empty;

        egui::ComboBox::from_label("Scenario")
            .selected_text(format!("{selected:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, Scenario::Empty, "Empty");
                ui.selectable_value(&mut selected, Scenario::HydrogenHalide, "Hydrogen Halide");
            });
    });
}

