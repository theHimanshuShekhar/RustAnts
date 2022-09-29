use std::time::Duration;

use ant::AntPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::prelude::*;

mod ant;
mod components;

pub const PI: f32 = 3.14159265358979323846264338327950288f32;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// Window Size
const WINDOW_HEIGHT: f32 = 720.;
const WINDOW_WIDTH: f32 = 1280.;

// Global Data
struct GlobalSettings {
    ants_count: i32,
    ants_size: f32,
    move_speed: f32,
    wander_strength: f32,
    pheremone_life: i32,
    home_pheromone_color: [f32; 3],
    food_pheromone_color: [f32; 3],
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "RustANTS!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: true,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(GlobalSettings {
            move_speed: 100.,
            ants_count: 10,
            ants_size: 5.,
            wander_strength: 0.05,
            pheremone_life: 50,
            home_pheromone_color: [0.2, 0.6, 0.4],
            food_pheromone_color: [1.0, 0.35, 0.75],
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(AntPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: Duration::new(10, 0),
            ..Default::default()
        })
        .add_plugin(EguiPlugin)
        .add_system(menu_ui)
        .add_startup_system(setup_system)
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .run();
}

fn menu_ui(mut egui_context: ResMut<EguiContext>, mut settings: ResMut<GlobalSettings>) {
    egui::Window::new("Settings").show(egui_context.ctx_mut(), |ui| {
        ui.label("ants_count");
        ui.add(egui::DragValue::new(&mut settings.ants_count).speed(1));

        ui.label("ants_size");
        ui.add(egui::DragValue::new(&mut settings.ants_size).speed(1));

        ui.label("move_speed");
        ui.add(egui::DragValue::new(&mut settings.move_speed).speed(10));

        ui.label("wander_strength");
        ui.add(egui::DragValue::new(&mut settings.wander_strength).speed(0.01));

        ui.label("pheremone_life");
        ui.add(egui::DragValue::new(&mut settings.pheremone_life).speed(1));

        ui.label("home_pheromone_color");
        egui::color_picker::color_edit_button_rgb(ui, &mut settings.home_pheromone_color);

        ui.label("food_pheromone_color");
        egui::color_picker::color_edit_button_rgb(ui, &mut settings.food_pheromone_color);
    });
}
pub struct WinSize {
    width: f32,
    height: f32,
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    // 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Get Window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize {
        width: win_w,
        height: win_h,
    };
    commands.insert_resource(win_size)
}
