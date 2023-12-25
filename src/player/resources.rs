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
use rand::Rng;
use crate::Timer;
use robotics_lib::interface::where_am_i;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::interface::look_at_sky;
use robotics_lib::interface::get_score;
use robotics_lib::interface::destroy;

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
}

pub struct MyRobot(pub Robot, pub Mutex<Sender<((Vec<Vec<Option<Tile>>>, (usize, usize)), EnvironmentalConditions, f32)>>);

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        println!("CIAOOOO");

        /*
        // Go a direction
        let dir = Direction::Down;
         match go_allowed(self, &world, &dir) {
            Ok(_) => {
                self.go_ui(world, dir);
            },
            Err(_) => {},
        }
        */

        // GO in random directions 
        let mut rng = rand::thread_rng();
        let idir = rng.gen_range(0..4);
        let dir = match idir {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        };
        match go_allowed(self, &world, &dir) {
            Ok(_) => {
                self.go_ui(world, dir);
            },
            Err(_) => {},
        }

        // Pickup objects
        let _ = destroy(self, world, Direction::Up);
      
        /*
        // Do nothing
        let view = where_am_i(self, world);
        let _ = self.1.lock().unwrap().send(view);
        */

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

impl MyRobot {
    fn go_ui(&mut self, world: &mut World, direction: Direction) {
        let _ = go(self, world, direction);
        let view = where_am_i(self, world);
        let condition = look_at_sky(world);
        let score = get_score(world);
        let _ = self.1.lock().unwrap().send((view, condition, score));
    }
}
