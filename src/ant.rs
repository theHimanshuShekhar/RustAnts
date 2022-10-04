use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use min_max::*;
use rand::Rng;

use crate::{
    components::{Ant, Direction, Food, HasFood, Home, Pheromone, PheromoneType},
    GlobalSettings, WinSize, TIME_STEP,
};

use self::pheromone::PheromonePlugin;
pub struct AntPlugin;
mod pheromone;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ant_spawn_system)
            .add_system(pheromone_update_system)
            .add_system(food_update_system)
            .add_system(home_update_system)
            .add_system(ant_update_system)
            .add_plugin(PheromonePlugin);
    }
}

fn ant_spawn_system(
    mut commands: Commands,
    settings: ResMut<GlobalSettings>,
    mut query: Query<&bevy::prelude::Transform, With<Home>>,
) {
    // Spawn Ant shape
    let shape = shapes::Circle {
        radius: settings.ants_size,
        ..Default::default()
    };

    for transform in query.iter_mut() {
        for _ in 0..settings.ants_count / settings.home_count {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::WHITE),
                        outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 1.),
                    },
                    bevy::prelude::Transform::default(),
                ))
                .insert(Ant)
                .insert(Direction {
                    angle: rand::thread_rng().gen_range(-360.0..360.0),
                })
                .insert(HasFood {
                    has_food: rand::thread_rng().gen_bool(0.),
                })
                .insert_bundle(SpatialBundle {
                    transform: *transform,
                    visibility: Visibility { is_visible: true },
                    ..Default::default()
                });
        }
    }
}

fn pheromone_update_system(
    mut query: Query<(&mut Direction, &mut Transform, &HasFood), (With<Ant>, Without<Pheromone>)>,
    pheromone_query: Query<(&Transform, &PheromoneType), With<Pheromone>>,
    settings: ResMut<GlobalSettings>,
) {
    for (mut direction, mut transform, has_food) in query.iter_mut() {
        let translation = &mut transform.translation;
        let angle = &mut direction.angle;
        let hasfood = has_food.has_food;

        for (pheromone_transform, pheromone_type) in pheromone_query.iter() {
            let pheromone_position = pheromone_transform.translation;
            let ant_position = *translation;

            if ant_position.distance(pheromone_position) <= settings.detection_radius {
                if hasfood && pheromone_type.pheromone_type == "home" {
                    *angle = ant_position.angle_between(pheromone_position).to_degrees();
                    break;
                } else if !hasfood && pheromone_type.pheromone_type == "food" {
                    *angle = ant_position.angle_between(pheromone_position).to_degrees();
                    break;
                }
            }
        }
    }
}

fn food_update_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut HasFood), (With<Ant>, Without<Food>)>,
    food_query: Query<(&Transform, Entity), With<Food>>,
    settings: ResMut<GlobalSettings>,
) {
    for (mut transform, mut has_food) in query.iter_mut() {
        let translation = &mut transform.translation;
        let hasfood = &mut has_food.has_food;

        for (food_transform, food) in food_query.iter() {
            if !*hasfood {
                let food_position = food_transform.translation;
                let ant_position = *translation;

                if ant_position.distance(food_position) <= settings.detection_radius {
                    *hasfood = true;
                    commands.entity(food).despawn();
                    break;
                }
            }
        }
    }
}

fn home_update_system(
    mut query: Query<(&mut Transform, &mut HasFood), (With<Ant>, Without<Home>)>,
    food_query: Query<&Transform, With<Home>>,
    settings: ResMut<GlobalSettings>,
) {
    for (mut transform, mut has_food) in query.iter_mut() {
        let translation = &mut transform.translation;
        let hasfood = &mut has_food.has_food;

        for home_transform in food_query.iter() {
            if !*hasfood {
                let food_position = home_transform.translation;
                let ant_position = *translation;

                if ant_position.distance(food_position) <= settings.detection_radius {
                    *hasfood = false;
                    break;
                }
            }
        }
    }
}

fn ant_update_system(
    mut ant_query: Query<(&mut Transform, &mut Direction), With<Ant>>,
    settings: ResMut<GlobalSettings>,
    win_size: Res<WinSize>,
) {
    for (mut transform, mut direction) in ant_query.iter_mut() {
        let translation = &mut transform.translation;
        let angle = &mut direction.angle;

        let max_x: f32 = win_size.width / 2. - settings.ants_size;
        let min_x: f32 = -win_size.width / 2. + settings.ants_size;
        let max_y: f32 = win_size.height / 2. - settings.ants_size;
        let min_y: f32 = -win_size.height / 2. + settings.ants_size;

        // Update position and angle of ant

        // Convert from radians to degree
        let x_change = angle.cos();
        let y_change = angle.sin();

        translation.x += x_change * TIME_STEP * settings.move_speed;
        translation.y += y_change * TIME_STEP * settings.move_speed;
        *angle = *angle + rand::thread_rng().gen_range(-1.0..1.0) * settings.wander_strength;

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
