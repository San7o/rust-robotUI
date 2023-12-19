use bevy::prelude::*;
use crate::world::systems::render_map;

pub mod components;
pub mod systems;
pub mod resources;
pub mod generator;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, render_map);
    }
}
