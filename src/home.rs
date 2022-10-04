use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::{components::Home, GlobalSettings, WinSize};

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(home_spawn_system);
    }
}

fn home_spawn_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    settings: ResMut<GlobalSettings>,
) {
    let home_size = 20.;
    let min_distance_from_edges = 50.;
    let max_x: f32 = win_size.width / 2. - home_size - min_distance_from_edges as f32;
    let min_x: f32 = -win_size.width / 2. + home_size + min_distance_from_edges as f32;
    let max_y: f32 = win_size.height / 2. - home_size - min_distance_from_edges as f32;
    let min_y: f32 = -win_size.height / 2. + home_size + min_distance_from_edges as f32;

    let shape = shapes::Circle {
        radius: settings.home_radius,
        ..Default::default()
    };

    for _ in 0..settings.home_count {
        let home_x = rand::thread_rng().gen_range(min_x..max_x);
        let home_y = rand::thread_rng().gen_range(min_y..max_y);

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::TURQUOISE),
                    outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 1.),
                },
                Transform::default(),
            ))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(home_x, home_y, 0.1),
                visibility: Visibility { is_visible: true },
                ..Default::default()
            })
            .insert(Home);
    }
}
