use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::components::Player;

pub const PLAYER_SIZE : f32 = 32.0;

// This is a system
pub fn spawn_player(
    // To spawn entity
    mut commands: Commands,
    // To get information about width or height
    window_query : Query<&Window, With<PrimaryWindow>>,
    // Acces to the asset server to access all the files. Those are created by bevy 
    // Res is a resource, It's a unique and globally accessible struct 
    // Onlt one Resource<T> of type T can exist at one time 
    asset_server : Res<AssetServer>,
) {
    // Get a reverence to our window 
    // There is onlt one window which is the primary window 
    // so instead of an iterator, we use get_single() 
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            // We are spawning a SpriteBundle
            // We use bundles to quickly add or remove sets of Components 
            // to or form an Entiy 
            // To know: Do not use bundles as queries! 
            SpriteBundle {
                // transform : Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                transform : Transform::from_xyz(0.0, 0.0, 1.0),
                texture: asset_server.load("robot_pixelart.png"), 
                ..default()
            },
            Player{},
        )
    );
}
