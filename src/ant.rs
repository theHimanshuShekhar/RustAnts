use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::{
    components::{Ant, Velocity},
    WinSize, BASE_SPEED, TIME_STEP,
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
        radius: 10.,
        ..Default::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::DARK_GRAY, 0.5),
            },
            Transform::default(),
        ))
        .insert(Ant)
        .insert(Velocity { x: 1., y: 1. });
}

fn ant_update_system(
    mut query: Query<(&Velocity, &mut Transform), With<Ant>>,
    win_size: Res<WinSize>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        // // Left edge
        // if translation.x <= 1. {
        //     translation.x += 1.;
        // }
        // // Right edge
        // else if translation.x > win_size.width - 1. {
        //     translation.x -= 1.;
        // }
        // // Top edge
        // else if translation.x >= 1. {
        //     translation.y += 1.;
        // }
        // // Bottom edge
        // else if translation.x > win_size.height - 1. {
        //     translation.y -= 1.;
        // }
        // // Move Randomly
        // else {
        let mut rng = rand::thread_rng();
        translation.x += velocity.x * rng.gen_range(-1.0..1.0) * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * rng.gen_range(-1.0..1.0) * TIME_STEP * BASE_SPEED;
        // }
    }
}
