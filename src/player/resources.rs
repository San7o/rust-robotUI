use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::Robot;
use robotics_lib::world::World;
use robotics_lib::interface::Direction;
use robotics_lib::interface::go;
use robotics_lib::utils::go_allowed;
use bevy::ecs::system::Resource;
use crate::Component;
use robotics_lib::world::tile::Tile;
use robotics_lib::interface::robot_map;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

pub struct MyRobot(pub Robot, pub Mutex<Sender<Vec<Vec<Option<Tile>>>>>);

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        println!("CIAOOOO");


        match go_allowed(self, &world, &Direction::Down) {
            Ok(_) => {
                go(self, world, Direction::Down);
            },
            Err(_) => {},
        }

         let map = robot_map(&world); 
        self.1.lock().unwrap().send(map.unwrap());

        


    }

    fn handle_event(&mut self, event: Event) {
        println!("{:?}", event);
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }
}
