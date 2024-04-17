use bevy::prelude::*;

use crate::{game::SimulationState, AppState};

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            println!("Entered AppState::Game");
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            println!("Entered AppState::MainMenu");
            next_simulation_state.set(SimulationState::Paused);
            next_app_state.set(AppState::MainMenu);
        }
    }
}
