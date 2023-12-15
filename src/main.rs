use bevy::prelude::*;
use crate::camera::systems::spawn_camera;
use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;
use bevy_ecs_tilemap::TilemapPlugin;

mod player;
mod camera;
mod systems;
mod world;
mod custom_robot;
mod custom_world_generator;

use robotics_lib::world::world_generator;
use player::PlayerPlugin;
use crate::custom_world_generator::WorldGenerator;
use crate::custom_robot::MyRobot;

use robotics_lib::runner::Runner;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Robot;


fn main() {
    
    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let tools = vec![Tool];
    let mut generator = WorldGenerator::init(10); // Map size
    let run = Runner::new(Box::new(r), &mut generator); // TODO: link tools

    /*
    //Known bug: 'check_world' inside 'Runner::new()' fails every time
    match run {
        | Ok(mut r) => {
            for _ in 0..19 {
                let _ = r.game_tick();
            }
        }
        | Err(e) => println!("{:?}", e),
    }
    */


    App::new()
        // resources 
        .init_resource::<WorldRes>()
        // Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Clear pixels
        .add_plugins(TilemapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run();

}
