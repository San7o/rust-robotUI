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
use crate::UI::UIPlugin;

mod player;
mod camera;
mod systems;
mod world;
mod resources;
mod main_menu;
mod UI;

use robotics_lib::world::world_generator;
use player::PlayerPlugin;
use crate::world::resources::WorldRes;

use robotics_lib::runner::Runner;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Robot;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::world::world_generator::Generator;
use oxagworldgenerator::world_generator::content_options::OxAgContentOptions;
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use robotics_lib::world::tile::Content;

use std::sync::mpsc;
use std::thread;
use std::sync::Mutex;

// use worldgen_unwrap::public::WorldgeneratorUnwrap;
use std::path::PathBuf;

const WORLD_SIZE : usize = 300;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Pause,
}

fn main() {
    
    // Creating the channel from the Runner to the ECS
    let (tx, rx) = mpsc::channel::<((Vec<Vec<Option<Tile>>>, (usize, usize)), EnvironmentalConditions, f32)>();
    // Creating an empty world resource
    let wr = WorldRes {
        world: None,
        rx: Mutex::new(rx),
        player_x: 0,
        player_y: 0,
        world_size: WORLD_SIZE,
        environmental_conditions: EnvironmentalConditions::new(&vec![WeatherType::Sunny], 0, 0).unwrap(), // Just as tmp
        score: 0.0,
        elevation: 0,
    };

    // Creating the robot and the Runner
    let r = MyRobot(Robot::new(), Mutex::new(tx));
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];


    // Generating the world 
    
    // Simple generator  
    //let mut generator = WorldGenerator::init(WORLD_SIZE);
    
    // --- Actual generator
   
    /*
    // Generate 
    let seed = 751776;
    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(WORLD_SIZE)
        .set_environmental_conditions_from_preset(OxAgEnvironmentalConditionPresets::Sunny)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_with_info(true)
        .build()
        .unwrap();

    //let mut generator = generator.gen();
    generator.save("worlds/save.json").unwrap();
    */ 

    // Load 
    let mut generator = OxAgWorldGeneratorBuilder::new()
        .load("worlds/save.json")
        .unwrap();
   
    // Unwrap generator
    //let mut generator = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("large_world.bin")));



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
        .add_plugins(UIPlugin)
        // Systems 
        //.add_systems(Startup, background_music)
        .add_systems(Update, 
            (
                exit_game,
                transition_to_game_state,
            )
        ).run();
}
