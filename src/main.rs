#![allow(unused_imports)]

use bevy::prelude::*;
use crate::camera::systems::spawn_camera;
use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy::log::LogPlugin;
use crate::world::generator::WorldGenerator;
use crate::player::resources::MyRobot;

mod player;
mod camera;
mod systems;
mod world;
mod resources;
// mod winit;

use robotics_lib::world::world_generator;
use player::PlayerPlugin;
use crate::world::resources::WorldRes;

use robotics_lib::runner::Runner;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Robot;
use robotics_lib::world::tile::Tile;

use std::sync::mpsc;
use std::thread;
use std::sync::Mutex;

fn main() {
    
    let (tx, rx) = mpsc::channel::<Vec<Vec<Option<Tile>>>>();
    let wr = WorldRes {
        world: None,
        rx: Mutex::new(rx),
        player_x: 0,
        player_y: 0,
    };

    let r = MyRobot(Robot::new(), Mutex::new(tx));
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    let mut generator = WorldGenerator::init(20); // Map size
    let run = Runner::new(Box::new(r), &mut generator).unwrap(); // TODO: link tools

    App::new()
        // Resources 
        .insert_resource(wr)
        .insert_non_send_resource(run)
        // Plugins
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()) // Clear Pixels
            .set(LogPlugin {
            filter: "mygame=debug".into(),
            level: bevy::log::Level::ERROR,})
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Robot UI"),
                        ..Default::default()
                    }),
                    ..default()
                })
            )
        .add_plugins(CameraPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run();

}
