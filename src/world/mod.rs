use bevy::prelude::*;
use crate::world::systems::spawn_map;
use crate::world::systems::update_map;

pub mod components;
pub mod systems;
pub mod resources;
pub mod generator;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map)
            .add_systems(Update, update_map);
    }
}
