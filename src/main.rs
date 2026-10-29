use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod app_controller;
mod gizmo_renders;
mod rail_types;
mod setup;

use app_controller::*;
use gizmo_renders::*;
use setup::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Play,
    Stop,
}

pub fn appstate_toggler(
    input: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        match state.get() {
            AppState::Play => next_state.set(AppState::Stop),
            AppState::Stop => next_state.set(AppState::Play),
        }
    }
}

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(AppState::Stop)))
        // states
        .init_state::<AppState>()
        // systems
        .add_systems(Startup, setup)
        .add_systems(Update, appstate_toggler)
        .add_systems(Update, place_rail_tile.run_if(in_state(AppState::Play)))
        .add_systems(Update, (draw_grid, draw_ports, draw_nodes))
        .add_systems(
            FixedUpdate,
            camera_controls.run_if(in_state(AppState::Play)),
        )
        .add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
        // resources
        .run();
}
