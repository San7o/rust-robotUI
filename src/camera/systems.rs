use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use crate::camera::components::CameraMaker;

pub fn zoom_scalingmode(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<CameraMaker>>,
) { 

    if keyboard_input.just_pressed(KeyCode::Plus) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale /= 1.25;
    }

    if keyboard_input.just_pressed(KeyCode::Minus) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale *= 1.25;
    }

 }


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    
    commands.spawn(
        (
            Camera2dBundle {
                // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            CameraMaker{},
        )
    );
}

