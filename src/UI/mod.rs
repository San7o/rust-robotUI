use bevy::prelude::*;
use crate::AppState;
use systems::layout::*;
use crate::UI::systems::interactions::*;

mod components;
mod styles;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_ui_menu)
           .add_systems(OnExit(AppState::Game), despawn_ui_menu) // Might have to remove this for
                                                                 // pause state
           .add_systems(Update,
                (
                    update_bottom_bar,
                )
                .run_if(in_state(AppState::Game))
            )
            ;
    }
}


