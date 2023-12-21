use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::TilemapTexture;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_ecs_tilemap::prelude::TileStorage;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::prelude::TilemapId;
use bevy_ecs_tilemap::prelude::TilemapTileSize;
use bevy_ecs_tilemap::prelude::TilemapType;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::Content;
use robotics_lib::interface::robot_map;
use bevy::prelude::Resource;
use crate::world::resources::WorldRes;
use robotics_lib::runner::Runnable;
use crate::world::components::TileDraw;
use crate::world::components::GridDraw;

pub fn spawn_map(
    mut commands: Commands,
    mut world: ResMut<WorldRes>,
    rendered_map: Query<Entity, (With<TileDraw>, With<GridDraw>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader,>,
) {
        
    // TODO This is too slow
    /*
    let map1 = match world.rx.lock().unwrap().try_recv() {
         Ok(w) => Some(w),
         Err(_) => None,
    };
    

    let tile = Tile {
        tile_type: TileType::Grass,
        content:  Content::Garbage(2),
        elevation: 20,
    };

    if let None = map1 {
        return;
    }
    let map = &map1.clone().unwrap();

    let map_size = map.len();
    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let texture_index = TileTextureIndex(match &map[x as usize][y as usize] {
                Some(tile) => {
                    match tile.tile_type {
                        TileType::Grass => 0,
                        TileType::Sand => 1,
                        _ => 3,
                    }
                },
                None => 3,
            }); 
            // Make all the textures
            // todo!();

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: texture_index,
                    ..Default::default()
                },
                TileDraw{}))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    */ 
    
    

    let window = window_query.get_single().unwrap();
    let texture_handle: Handle<Image> = asset_server.load("tiles2.png");

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    let map_size = world.world_size;
    let map_size = TilemapSize { x: map_size as u32, y: map_size as u32};

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn empty map 
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(3), // Here the None texture index 
                    ..Default::default()
                },
                TileDraw{}))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }


    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    
    // Despawn ond maps
    for e in &rendered_map {
        commands.entity(e).despawn();
    }
    // Spawn new map
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    },
    GridDraw{}));

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("grass_pixelart.png")),
            tile_size,
            ..Default::default()
        });
    }
}


pub fn update_map(
    mut commands: Commands,
    mut world: ResMut<WorldRes>,
    mut tile_storage: Query<&mut TileStorage>,
    mut tilemap_entity_query: Query<Entity, With<GridDraw>>,
    rendered_map: Query<Entity, (With<TileDraw>, With<GridDraw>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader,>,
) {
        

    // Recieve the map or return
    let res = match world.rx.lock().unwrap().try_recv() {
         Ok(w) => Some(w),
         Err(_) => None,
    };
    if let None = res {
        return;
    }
    let (map, (robot_x, robot_y)) = res.unwrap();
    let map_size = world.world_size;

    // Dummy tile
    let tile = Tile {
        tile_type: TileType::Grass,
        content:  Content::Garbage(2),
        elevation: 20,
    };
    

    let texture_handle: Handle<Image> = asset_server.load("tiles2.png");

    // TODO Add multiple layers (content)
    let mut tile_map = &mut tile_storage.get_single_mut().unwrap();
    let tilemap_entity = tilemap_entity_query.get_single().unwrap();


    // Doing this for all nine tiles
   // Remember that the robot coordinates as swapped
   // ROBOT 
   // ------------------> y 
   // |
   // |
   // |
   // |
   // |
   // V 
   // x
   //
   // WORLD 
   // y 
   // ^
   // |
   // |
   // |
   // |
   // |
   // |
   // -------------------> x 

    // TOP LEFT
    if is_inside(robot_x as i32 - 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // TOP
    if is_inside(robot_x as i32 - 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // TOP RIGHT
    if is_inside(robot_x as i32 - 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // BOTTOM LEFT
    if is_inside(robot_x as i32 + 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // BOTTOM
    if is_inside(robot_x as i32 + 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // BOTTOM RIGHT
    if is_inside(robot_x as i32 + 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // MIDDLE LEFT
    if is_inside(robot_x as i32, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y: map_size as u32-1 - robot_x as u32,

        };
        let new_tile_id = get_texture_id(&map[1][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // MIDDLE RIGHT
    if is_inside(robot_x as i32, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id(&map[0][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
    
    // MIDDLE
    if is_inside(robot_x as i32, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id(&map[1][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }

}

// Using robot coordinates
fn is_inside(x: i32, y: i32, map_size: i32) -> bool {
    x >= 0 && y >= 0 && y < map_size && x < map_size 
}

// Gets the tilemap id given a TyleType 
fn get_texture_id(tile: &Option<Tile>) -> u32 {
    match tile {         
        Some(tile) => {
            match tile.tile_type {
                TileType::Grass => 0,
                TileType::Sand => 1,
                _ => 3,
            }
        },
        None => 3,
    }
    
}
