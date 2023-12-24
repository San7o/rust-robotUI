// use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{robot_map, Tools, where_am_i};
#[allow(unused_imports)]
use robotics_lib::interface::{craft, debug, destroy, go, look_at_sky, teleport, Direction, robot_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree};
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport,
};
#[allow(unused_imports)]
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;
use rand::Rng;

pub struct WorldGenerator {
    size: usize,
}
impl WorldGenerator {
    pub fn init(size: usize) -> Self {
        WorldGenerator { size }
    }
}
impl Generator for WorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {

        // Random Generator
        let mut rng = rand::thread_rng();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // Initialize the map with default tiles
        for x in 0..self.size {
            let mut row: Vec<Tile> = Vec::new();
            for y in 0..self.size {
                let i_tiletype = rng.gen_range(10..50);
                let i_content = rng.gen_range(0..5);
                //let i_tiletype = 3; // only grass
                //let i_content = 0;
                let tile_type = match i_tiletype {
                    | 0 => TileType::DeepWater,
                    | 1 => TileType::ShallowWater,
                    | 2 => TileType::Sand,
                    | 3 => TileType::Grass,
                    | 4 => TileType::Street,
                    | 5 => TileType::Hill,
                    | 6 => TileType::Mountain,
                    | 7 => TileType::Snow,
                    | 8 => TileType::Lava,
                    | 9 => TileType::Teleport(false),
                    | _ => TileType::Grass,
                };
                let content = match i_content {
                    | 0 => Rock(3),
                    | 1 => Tree(2),
                    | 2 => Garbage(2),
                    | 3 => Fire,
                    | 4 => Coin(2),
                    | 5 => Bin(2..3),
                    | 6 => Crate(2..3),
                    | 7 => Bank(3..54),
                    | 8 => Content::Water(20),
                    | 9 => Content::None,
                    | 10 => Fish(3),
                    | 11 => Market(20),
                    | 12 => Content::Building,
                    | 13 => Content::Bush(2),
                    | 14 => Content::JollyBlock(2),
                    | 15 => Content::Scarecrow,
                    | _ => Content::None,
                };
                row.push(Tile {
                    tile_type,
                    content,
                    elevation: 1,
                });
            }
            map.push(row);
        }
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

        let max_score = rand::random::<f32>();

        // Test Map
/*
        let mut map: Vec<Vec<Tile>> = Vec::new();
        self.size = 3;
       map.push(Vec::new());
       map.push(Vec::new());
       map.push(Vec::new());
        let top_left = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let top = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
          let top_right = Tile{
            tile_type: TileType::Grass,
            content: Content::Rock(1),
            elevation: 0,
        };
         let middle_left = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let middle = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let middle_right = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let bottom_left = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let bottom = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
         let bottom_right = Tile{
            tile_type: TileType::Grass,
            content: Content::None,
            elevation: 0,
        };
       map[0].push(top_left);
       map[0].push(top);
       map[0].push(top_right);
       map[1].push(middle_left);
       map[1].push(middle);
       map[1].push(middle_right);
       map[2].push(bottom_left);
       map[2].push(bottom);
       map[2].push(bottom_right);
         let environmental_conditions = EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();

        let max_score = rand::random::<f32>();
*/ 


        (map, (self.size / 2, self.size / 2), environmental_conditions, max_score, None)
    }
}

