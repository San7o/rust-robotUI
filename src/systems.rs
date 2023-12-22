use bevy::prelude::*;
use crate::AppState;
use bevy::app::AppExit;

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    // We use EventWriter<T> to send an event
    // adn EventReader<T> to receive events
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered Appstate::Game");
        }
    }
}
