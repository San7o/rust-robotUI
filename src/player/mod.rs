use bevy::prelude::*;
use crate::player::systems::spawn_player;
use crate::player::systems::tick_loop;


pub mod components;
pub mod systems;
pub mod resources;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems 
            .add_systems(Startup, spawn_player)
            .add_systems(Update, tick_loop);
    }
}
