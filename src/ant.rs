use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use min_max::*;
use rand::Rng;

use crate::{
    components::{Ant, Direction, Food, Pheromone, PheromoneAge, ID},
    WinSize, ANTS_COUNT, ANT_SIZE, MOVE_SPEED, PHEROMONE_LIFE, TIME_STEP, WANDER_STRENGTH,
};

struct TrailSpawnTimer(Timer);
struct TrailDespawnTimer(Timer);

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ant_spawn_system)
            .insert_resource(TrailSpawnTimer(Timer::from_seconds(0.05, true)))
            .insert_resource(TrailDespawnTimer(Timer::from_seconds(0.05, true)))
            .add_system(ant_update_system)
            .add_system(trail_spawn_system)
            .add_system(pheromone_update_system);
    }
}

fn ant_spawn_system(mut commands: Commands) {
    // Spawn Ant shape
    let shape = shapes::Circle {
        radius: ANT_SIZE,
        ..Default::default()
    };

    for _ in 0..ANTS_COUNT {
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

fn trail_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TrailSpawnTimer>,
    mut query: Query<(&mut Food, &mut Transform), With<Ant>>,
) {
    // Spawn Pheromone shape
    let shape = shapes::Circle {
        radius: 3.,
        ..Default::default()
    };
    if timer.0.tick(time.delta()).just_finished() {
        for (mut food, mut transform) in query.iter_mut() {
            let translation = &mut transform.translation;
            let _has_food = &mut food.has_food;

            // println!(
            //     "x:{},y:{} | hasfood:{}",
            //     translation.x, translation.y, has_food
            // );

            let pheremone_id = commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::LIME_GREEN),
                        outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 0.0),
                    },
                    Transform::from_xyz(translation.x, translation.y, 0.),
                ))
                .insert(Pheromone)
                .insert(PheromoneAge { age: 0 })
                .id();

            commands.entity(pheremone_id).insert(ID {
                id: pheremone_id.id(),
            });
        }
    }
}

fn pheromone_update_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PheromoneAge), With<Pheromone>>,
    time: Res<Time>,
    mut timer: ResMut<TrailDespawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (pheromone, mut pheromone_age) in query.iter_mut() {
            let age = &mut pheromone_age.age;

            if *age >= PHEROMONE_LIFE {
                commands.entity(pheromone).despawn();
            } else {
                *age += 1;
            }
        }
    }
}
