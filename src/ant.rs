use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use min_max::*;
use rand::Rng;
use random_color::RandomColor;

use crate::{
    components::{Ant, Direction},
    WinSize, ANTS_COUNT, ANT_SIZE, MOVE_SPEED, TIME_STEP, WANDER_STRENGTH,
};

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ant_spawn_system)
            .add_system(ant_update_system);
    }
}

fn ant_spawn_system(mut commands: Commands) {
    // Spawn Ant shape
    let shape = shapes::Circle {
        radius: ANT_SIZE,
        ..Default::default()
    };

    for _ in 0..ANTS_COUNT {
        // let random_color = RandomColor::new().alpha(1.).to_hsl_array();
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(
                        Color::BLACK,
                        // bevy::prelude::Color::hsl(
                        // random_color[0] as f32,
                        // random_color[1] as f32,
                        // random_color[2] as f32,
                        // )
                    ),
                    outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 1.),
                },
                Transform::default(),
            ))
            .insert(Ant)
            .insert(Direction {
                angle: rand::thread_rng().gen_range(0.0..360.0),
            })
            .insert_bundle(SpatialBundle {
                transform: Transform::from_scale(Vec3::splat(1.0)),
                visibility: Visibility { is_visible: true },
                ..Default::default()
            });
    }
}

fn ant_update_system(
    mut query: Query<(&mut Direction, &mut Transform), With<Ant>>,
    win_size: Res<WinSize>,
) {
    for (mut direction, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let angle = &mut direction.angle;

        let max_x: f32 = win_size.width / 2. - ANT_SIZE;
        let min_x: f32 = -win_size.width / 2. + ANT_SIZE;
        let max_y: f32 = win_size.height / 2. - ANT_SIZE;
        let min_y: f32 = -win_size.height / 2. + ANT_SIZE;

        let x_change = angle.cos();
        let y_change = angle.sin();

        // Update position and angle of ant
        translation.x = translation.x + x_change * TIME_STEP * MOVE_SPEED;
        translation.y = translation.y + y_change * TIME_STEP * MOVE_SPEED;
        *angle = *angle + rand::thread_rng().gen_range(-5.0..5.0) * WANDER_STRENGTH;

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
