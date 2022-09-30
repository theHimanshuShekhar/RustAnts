use bevy::prelude::Component;

#[derive(Component)]
pub struct HasFood {
    pub has_food: bool,
}

#[derive(Component)]
pub struct Direction {
    pub angle: f32,
}

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct PheromoneAge {
    pub age: i32,
}
#[derive(Component)]
pub struct Pheromone;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Home;
