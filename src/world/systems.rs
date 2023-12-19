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

pub fn render_map(
    mut commands: Commands,
    mut world: ResMut<WorldRes>,
    rendered_map: Query<Entity, (With<TileDraw>, With<GridDraw>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader,>,
) {
        
    // TODO This is too slow

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

    let window = window_query.get_single().unwrap();
    let texture_handle: Handle<Image> = asset_server.load("tiles2.png");

    let map_size = TilemapSize { x: map_size as u32, y: map_size as u32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

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
