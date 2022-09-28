use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use min_max::*;
use rand::Rng;

use crate::{
    components::{Ant, Direction},
    WinSize, ANTS_COUNT, MOVE_SPEED, TIME_STEP,
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

    for _ in 0..ANTS_COUNT {
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

        let max_x: f32 = win_size.width / 2.;
        let min_x: f32 = -win_size.width / 2.;
        let max_y: f32 = win_size.height / 2.;
        let min_y: f32 = -win_size.height / 2.;

        let x_change = angle.cos();
        let y_change = angle.sin();

        translation.x = translation.x + x_change * TIME_STEP * MOVE_SPEED;
        translation.y = translation.y + y_change * TIME_STEP * MOVE_SPEED;

        if translation.x < min_x + 5.
            || translation.x >= max_x - 5.
            || translation.y < min_y + 5.
            || translation.y >= max_y - 5.
        {
            translation.x = min_partial!(
                win_size.width / 2. - 1.,
                max_partial!(-win_size.width / 2., translation.x)
            );
            translation.y = min_partial!(
                win_size.height / 2. - 1.,
                max_partial!(-win_size.height / 2., translation.y)
            );
            *angle = rand::thread_rng().gen_range(0.0..360.0);
        }
    }
}

// Clamp position to map boundaries, and pick new random move dir if hit boundary
// if (newPos.x < 0 || newPos.x >= width || newPos.y < 0 || newPos.y >= height) {
//     random = hash(random);
//     float randomAngle = scaleToRange01(random) * 2 * 3.1415;

//     newPos.x = min(width-1,max(0, newPos.x));
//     newPos.y = min(height-1,max(0, newPos.y));
//     agents[id.x].angle = randomAngle;
// }
// else {
//     int2 coord = int2(newPos);
//     float4 oldTrail = TrailMap[coord];
//     TrailMap[coord] = min(1, oldTrail + agent.speciesMask * trailWeight * deltaTime);
// }

// agents[id.x].position = newPos;
