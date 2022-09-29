use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{Ant, Food, Pheromone, PheromoneAge},
    PHEROMONE_LIFE,
};

struct TrailSpawnTimer(Timer);
struct TrailDespawnTimer(Timer);

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrailSpawnTimer(Timer::from_seconds(0.05, true)))
            .insert_resource(TrailDespawnTimer(Timer::from_seconds(0.05, true)))
            .add_system(trail_spawn_system)
            .add_system(pheromone_update_system);
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

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::LIME_GREEN),
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
