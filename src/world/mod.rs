use bevy::prelude::*;
use crate::world::systems::spawn_world;

mod components;
mod systems;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_world);
//            .add_systems(Update, render_world);
    }
}
