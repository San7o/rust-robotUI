use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::components::Player;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::Runner;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Robot;
use std::sync::Arc;
use crate::WorldRes;
use bevy::time::Timer;
use std::time::Duration;
use crate::player::resources::TickTimer;
use bevy::prelude::Transform;

// pub const PLAYER_SIZE : f32 = 32.0;

// TODO Those only work in a 10 by 10 map
const FACTOR :f32 = 16.0;
const TRANSLATE_Y : f32 = -72.0;
const TRANSLATE_X : f32 = -72.0;


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
    // The timer for the ticks
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
                transform : Transform::from_xyz(5.0, 0.0, 1.0),
                texture: asset_server.load("robot_pixelart.png"), 
                ..default()
            },
            Player{},
        ),
    );

    commands.insert_resource(TickTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    );
}


pub fn move_player(
    world_res: Res<WorldRes>,
    // Coordinate globali
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = &mut player_query.get_single_mut().unwrap();
    
    // Old coordinates
    let old_x = player.translation.x;
    let old_y = player.translation.y;
    // New Coordinates
    let new_x = world_res.player_x;
    let new_y = world_res.player_y;
    println!("NEw x: {}, New y: {}, oldx; {}, old_y: {}", new_x, new_y, old_x, old_y);

    if old_x != new_x as f32 * FACTOR + TRANSLATE_X || old_y != new_y as f32 * FACTOR + TRANSLATE_Y {
    // Calculate the translation
        // TODO Accound for matrix world position 
    // Disabled tranition
        player.translation.x = new_x as f32 * FACTOR + TRANSLATE_X;
        player.translation.y = new_y as f32 * FACTOR + TRANSLATE_Y;

    }
  }

pub fn tick_loop (
    mut world_res: ResMut<WorldRes>,
    mut runner_query: NonSendMut<Runner>,
    mut timer: ResMut<TickTimer>,
    time: Res<Time>,
) {
    
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        runner_query.game_tick();
        
        let robot = runner_query.get_robot();
        // Update player coordinates
        let coordinates = robot.get_coordinate();
        world_res.player_x = coordinates.get_row();
        world_res.player_y = coordinates.get_col();
    }
}
