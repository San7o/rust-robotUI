use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldRes {
    world: &World,
    robot: &impl Runnable,
}
