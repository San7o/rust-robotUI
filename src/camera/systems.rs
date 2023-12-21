use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use crate::camera::components::CameraMaker;
use crate::MyRobot;
use crate::player::components::Player;

pub fn zoom_scalingmode(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<CameraMaker>>,
) { 

    if keyboard_input.pressed(KeyCode::Plus) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale /= 1.03;
    }

    if keyboard_input.pressed(KeyCode::Minus) {
        let mut projection = camera_query.single_mut();
        // Zoom in 
        projection.scale *= 1.03;
    }

 }

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    // Zooming 
    camera.projection.scale /= 3.0;
    commands.spawn(
        (
            camera,   
            CameraMaker{},
        )
    );
}

pub fn follow_robot(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_transform_query: Query<&mut Transform, With<CameraMaker>>,
    robot_transform_query: Query<&Transform, (With<Player>, Without<CameraMaker>)>,
) {
    if keyboard_input.pressed(KeyCode::F) {
        let mut camera_transform = camera_transform_query.get_single_mut().unwrap();
        let robot_transform = robot_transform_query.get_single().unwrap();

        camera_transform.translation.x = robot_transform.translation.x;
        camera_transform.translation.y = robot_transform.translation.y;
    }
}
