use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{Ant, HasFood, Pheromone, PheromoneAge},
    GlobalSettings, TIME_STEP,
};

struct TrailSpawnTimer(Timer);
struct TrailDespawnTimer(Timer);

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrailSpawnTimer(Timer::from_seconds(TIME_STEP * 2., true)))
            .insert_resource(TrailDespawnTimer(Timer::from_seconds(TIME_STEP * 2., true)))
            .add_system(trail_spawn_system)
            .add_system(pheromone_update_system);
    }
}

fn trail_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TrailSpawnTimer>,
    mut query: Query<(&mut HasFood, &mut Transform), With<Ant>>,
    settings: ResMut<GlobalSettings>,
) {
    // Spawn Pheromone shape
    let shape = shapes::Circle {
        radius: 3.,
        ..Default::default()
    };
    if timer.0.tick(time.delta()).just_finished() {
        for (mut food, mut transform) in query.iter_mut() {
            let translation = &mut transform.translation;
            let has_food = &mut food.has_food;

            // println!(
            //     "x:{},y:{} | hasfood:{}",
            //     translation.x, translation.y, has_food
            //     settings.home_pheromone_color
            // );

            let pheromone_color = if *has_food {
                bevy::prelude::Color::rgba(
                    settings.food_pheromone_color[0],
                    settings.food_pheromone_color[1],
                    settings.food_pheromone_color[2],
                    0.2,
                )
            } else {
                bevy::prelude::Color::rgba(
                    settings.home_pheromone_color[0],
                    settings.home_pheromone_color[1],
                    settings.home_pheromone_color[2],
                    0.2,
                )
            };

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(pheromone_color),
                        outline_mode: StrokeMode::new(bevy::prelude::Color::BLACK, 0.0),
                    },
                    Transform::from_xyz(translation.x, translation.y, 0.),
                ))
                .insert(Pheromone)
                .insert(PheromoneAge { age: 0 });
        }
    }
}

fn pheromone_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TrailDespawnTimer>,
    mut query: Query<(Entity, &mut PheromoneAge), With<Pheromone>>,
    settings: ResMut<GlobalSettings>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (pheromone, mut pheromone_age) in query.iter_mut() {
            let age = &mut pheromone_age.age;

            if *age >= settings.pheremone_life {
                commands.entity(pheromone).despawn();
            } else {
                *age += 1;
            }
        }
    }
}
