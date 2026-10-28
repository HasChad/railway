use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod app_controller;
mod gizmo_renders;
mod rail_types;
mod setup;

use app_controller::*;
use gizmo_renders::*;
use setup::*;

use crate::rail_types::RailNode;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Tab)))
        // systems
        .add_systems(Startup, setup)
        .add_systems(Update, (place_rail_tile, draw_grid, draw_ports, draw_nodes))
        .add_systems(FixedUpdate, camera_controls)
        .add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
        // resources
        .init_resource::<RailNode>()
        .run();
}
