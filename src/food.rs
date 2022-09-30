use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::{components::Food, GlobalSettings, WinSize};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(food_spawn_system);
    }
}

fn food_spawn_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    settings: ResMut<GlobalSettings>,
) {
    // choose random locations for food depots and spawn

    let shape = shapes::Circle {
        radius: settings.ants_size,
        ..Default::default()
    };

    let max_x: f32 = win_size.width / 2. - settings.food_count_in_depot as f32;
    let min_x: f32 = -win_size.width / 2. + settings.food_count_in_depot as f32;
    let max_y: f32 = win_size.height / 2. - settings.food_count_in_depot as f32;
    let min_y: f32 = -win_size.height / 2. + settings.food_count_in_depot as f32;

    for _ in 0..settings.food_depot_count {
        let depot_x = rand::thread_rng().gen_range(min_x..max_x);
        let depot_y = rand::thread_rng().gen_range(min_y..max_y);

        for _ in 0..settings.food_count_in_depot {
            let variance_x = rand::thread_rng().gen_range(
                -shape.radius * (settings.food_count_in_depot as f32 * 0.1)
                    ..shape.radius * (settings.food_count_in_depot as f32 * 0.1),
            );
            let variance_y = rand::thread_rng().gen_range(
                -shape.radius * (settings.food_count_in_depot as f32 * 0.1)
                    ..shape.radius * (settings.food_count_in_depot as f32 * 0.1),
            );
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::GREEN),
                        outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 0.5),
                    },
                    Transform::default(),
                ))
                .insert_bundle(SpatialBundle {
                    transform: Transform::from_xyz(depot_x + variance_x, depot_y + variance_y, 0.1),
                    visibility: Visibility { is_visible: true },
                    ..Default::default()
                })
                .insert(Food);
        }
    }
}
