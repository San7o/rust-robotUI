use bevy::prelude::*;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), greetings);
    }
}

pub fn greetings() {
    println!("Welcome to the main menu!");
}
