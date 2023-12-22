use bevy::prelude::*;
use crate::player::systems::spawn_player;
use crate::player::systems::tick_loop;
use crate::player::systems::move_player;
use crate::world::systems::update_map;
use crate::player::systems::change_tick_speed;

pub mod components;
pub mod systems;
pub mod resources;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems 
            .add_systems(Startup, spawn_player)
            .add_systems(Update, tick_loop.after(update_map))
            .add_systems(Update, move_player.after(tick_loop))
            .add_systems(Update, change_tick_speed)
            ;
    }
}
