use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{Ant, HasFood, Pheromone, PheromoneAge, PheromoneType},
    GlobalSettings, TIME_STEP,
};

struct TrailSpawnTimer(Timer);
struct TrailDespawnTimer(Timer);

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrailSpawnTimer(Timer::from_seconds(TIME_STEP * 5.5, true)))
            .insert_resource(TrailDespawnTimer(Timer::from_seconds(
                TIME_STEP * 5.5,
                true,
            )))
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
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(2.),
        center: Vec2::new(0., 0.),
    };
    if timer.0.tick(time.delta()).just_finished() {
        for (food, mut transform) in query.iter_mut() {
            let translation = &mut transform.translation;
            let has_food = &food.has_food;

            let pheromone_color = if *has_food {
                bevy::prelude::Color::rgba(
                    settings.food_pheromone_color[0],
                    settings.food_pheromone_color[1],
                    settings.food_pheromone_color[2],
                    0.1,
                )
            } else {
                bevy::prelude::Color::rgba(
                    settings.home_pheromone_color[0],
                    settings.home_pheromone_color[1],
                    settings.home_pheromone_color[2],
                    0.1,
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
                .insert(PheromoneType {
                    pheromone_type: if *has_food {
                        String::from("food")
                    } else {
                        String::from("home")
                    },
                })
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
