use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use min_max::*;
use rand::Rng;

use crate::{
    components::{Ant, Direction, Food},
    GlobalSettings, WinSize, TIME_STEP,
};

use self::pheromone::PheromonePlugin;

pub struct AntPlugin;
mod pheromone;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ant_spawn_system)
            .add_system(ant_update_system)
            .add_plugin(PheromonePlugin);
    }
}

fn ant_spawn_system(mut commands: Commands, settings: ResMut<GlobalSettings>) {
    // Spawn Ant shape
    let shape = shapes::Circle {
        radius: settings.ants_size,
        ..Default::default()
    };

    for _ in 0..settings.ants_count {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::BLACK),
                    outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 1.),
                },
                Transform::default(),
            ))
            .insert(Ant)
            .insert(Direction {
                angle: rand::thread_rng().gen_range(0.0..360.0),
            })
            .insert(Food { has_food: false })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                visibility: Visibility { is_visible: true },
                ..Default::default()
            });
    }
}

fn ant_update_system(
    mut query: Query<(&mut Direction, &mut Transform), With<Ant>>,
    settings: ResMut<GlobalSettings>,
    win_size: Res<WinSize>,
) {
    for (mut direction, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let angle = &mut direction.angle;

        let max_x: f32 = win_size.width / 2. - settings.ants_size;
        let min_x: f32 = -win_size.width / 2. + settings.ants_size;
        let max_y: f32 = win_size.height / 2. - settings.ants_size;
        let min_y: f32 = -win_size.height / 2. + settings.ants_size;

        let x_change = angle.cos();
        let y_change = angle.sin();

        // Update position and angle of ant
        translation.x = translation.x + x_change * TIME_STEP * settings.move_speed;
        translation.y = translation.y + y_change * TIME_STEP * settings.move_speed;
        *angle = *angle + rand::thread_rng().gen_range(-5.0..5.0) * settings.wander_strength;

        // Clamp ant to be inside window size
        if translation.x < min_x
            || translation.x >= max_x
            || translation.y < min_y
            || translation.y >= max_y
        {
            translation.x = min_partial!(max_x, max_partial!(min_x, translation.x));
            translation.y = min_partial!(max_y, max_partial!(min_y, translation.y));
            *angle = rand::thread_rng().gen_range(0.0..360.0);
        }
    }
}
