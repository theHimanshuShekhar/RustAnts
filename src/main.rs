use ant::AntPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
// use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::prelude::*;

mod ant;
mod components;

pub const PI: f32 = 3.14159265358979323846264338327950288f32;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const MOVE_SPEED: f32 = 50.;
// const TURN_SPEED: f32 = 30. * 2. * PI;
const ANTS_COUNT: i32 = 10;
const ANT_SIZE: f32 = 10.;

// Window Size
const WINDOW_HEIGHT: f32 = 800.;
const WINDOW_WIDTH: f32 = 1280.;

const BACKGROUND_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

// fn menu_ui(mut egui_context: ResMut<EguiContext>) {
//     egui::Window::new("RustAnts!").show(egui_context.ctx_mut(), |ui| {
//         ui.label("Menu");
//     });
// }

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "RustANTS!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(AntPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(EguiPlugin)
        // .add_system(menu_ui)
        .add_startup_system(setup_system)
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .run();
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
