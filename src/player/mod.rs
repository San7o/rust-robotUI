use bevy::prelude::*;
use crate::player::systems::spawn_player;



pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems 
            .add_systems(Startup, spawn_player);
    }
}
