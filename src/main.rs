use bevy::prelude::*;

use crate::PlayerPlugin;

fn main() {
    
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        // Systems 
        .add_systems(Startup, spawn_camera)
        .run()

}
