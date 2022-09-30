use std::time::Duration;

use ant::AntPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use food::FoodPlugin;
use home::HomePlugin;
use menu::MenuPlugin;

mod ant;
mod components;
mod food;
mod home;
mod menu;

pub const PI: f32 = 3.14159265358979323846264338327950288f32;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// Window Size
const WINDOW_HEIGHT: f32 = 720.;
const WINDOW_WIDTH: f32 = 1280.;

pub struct WinSize {
    width: f32,
    height: f32,
}

// Global Data
struct GlobalSettings {
    ants_count: i32,
    ants_size: f32,
    move_speed: f32,
    wander_strength: f32,
    pheremone_life: i32,
    home_pheromone_color: [f32; 3],
    food_pheromone_color: [f32; 3],
    food_depot_count: i32,
    food_count_in_depot: i32,
    home_count: i32,
}

fn main() {
    App::new()
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
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
            food_depot_count: 4,
            food_count_in_depot: 100,
            home_count: 2,
        })
        .insert_resource(WinSize {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: Duration::new(10, 0),
            ..Default::default()
        })
        .add_plugin(EguiPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup_system)
        .add_plugin(HomePlugin)
        .add_plugin(AntPlugin)
        .add_plugin(FoodPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    // 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());
}
