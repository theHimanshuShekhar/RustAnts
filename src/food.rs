use bevy::prelude::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(food_spawn_system);
    }
}

fn food_spawn_system(mut commands: Commands) {
    // choose random locations for food depots and spawn
}
