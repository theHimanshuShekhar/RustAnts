use crate::GlobalSettings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_ui);
    }
}

fn menu_ui(mut egui_context: ResMut<EguiContext>, mut settings: ResMut<GlobalSettings>) {
    egui::Window::new("Settings").show(egui_context.ctx_mut(), |ui| {
        ui.label("ants_count");
        ui.add(egui::DragValue::new(&mut settings.ants_count).speed(1));

        ui.label("ants_size");
        ui.add(egui::DragValue::new(&mut settings.ants_size).speed(1));

        ui.label("food_depot_count");
        ui.add(egui::DragValue::new(&mut settings.food_depot_count).speed(1));

        ui.label("food_count_in_depot");
        ui.add(egui::DragValue::new(&mut settings.food_count_in_depot).speed(10));

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
