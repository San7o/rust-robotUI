use bevy::prelude::*;
use crate::camera::systems::spawn_camera;
use crate::camera::systems::zoom_scalingmode;
use crate::camera::systems::follow_robot;
use crate::camera::systems::camera_movement;

pub mod components;
pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup
            .add_systems(Startup, spawn_camera)
            // Update 
            .add_systems(Update, zoom_scalingmode)
            .add_systems(Update, follow_robot)
            .add_systems(Update, camera_movement)
            ;
    }
}
