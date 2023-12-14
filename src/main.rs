use bevy::prelude::*;
use crate::camera::systems::spawn_camera;
use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;
use bevy_ecs_tilemap::TilemapPlugin;

use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::Content;

mod player;
mod camera;
mod systems;
mod world;
use player::PlayerPlugin;

fn main() {
 
    let tile = Tile {
        tile_type: TileType::Grass,
        content:  Content::Garbage(2),
        elevation: 20,
    };

    let mut dummy_map: Vec<Vec<Option<Tile>>> = Vec::new();
    for i in 0..5 {
        dummy_map.push(Vec::new());
        dummy_map[i].push(Some(tile.clone()));
    }


    App::new()
        // Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Clear pixels
        .add_plugins(TilemapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run()

}
