use bevy::{prelude::*, window::PrimaryWindow};

use crate::{game::SimulationState, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && *app_state.get() != AppState::Game {
        next_app_state.set(AppState::Game);
        next_simulation_state.set(SimulationState::Running);
        println!("App in Game state.");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && *app_state.get() != AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
        println!("App in MainMenu state.");
    }
}
