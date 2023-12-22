#![allow(unused_imports)]

use bevy::prelude::*;
use crate::camera::systems::spawn_camera;
use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy::log::LogPlugin;
use crate::world::generator::WorldGenerator;
use crate::player::resources::MyRobot;
use crate::main_menu::MainMenuPlugin;
use crate::systems::transition_to_game_state;
use crate::systems::exit_game;
use crate::systems::background_music;
use bevy_kira_audio::AudioPlugin;

mod player;
mod camera;
mod systems;
mod world;
mod resources;
mod main_menu;

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

const WORLD_SIZE : usize = 20;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Pause,
}

fn main() {
    
    // Creating the channel from the Runner to the ECS
    let (tx, rx) = mpsc::channel::<(Vec<Vec<Option<Tile>>>, (usize, usize))>();
    // Creating an empty world resource
    let wr = WorldRes {
        world: None,
        rx: Mutex::new(rx),
        player_x: 0,
        player_y: 0,
        world_size: WORLD_SIZE,
    };

    // Creating the robot and the Runner
    let r = MyRobot(Robot::new(), Mutex::new(tx));
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    let mut generator = WorldGenerator::init(WORLD_SIZE);
    let run = Runner::new(Box::new(r), &mut generator).unwrap(); // TODO: link tools

    App::new()
        // Resources 
        .insert_resource(wr) // The World 
        .insert_non_send_resource(run) // The Runner, which cannot be passed in a thread safe way
        // States 
        .add_state::<AppState>()
        // Plugins
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())         // Clear Pixels
            .set(LogPlugin {
                    filter: "mygame=debug".into(),       // Less log spam 
                    level: bevy::log::Level::ERROR,
                })
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Robot UI"), // Title
                        ..Default::default()
                    }),
                    ..default()
                })
            )
        .add_plugins(AudioPlugin) // Bevy Kira Audio
        .add_plugins(MainMenuPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        // Systems 
        .add_systems(Startup, background_music)
        .add_systems(Update, 
            (
                exit_game,
                transition_to_game_state,
            )
        ).run();

}
