mod components;
mod plugins;
mod systems;

mod ui;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            plugins::CustomDefaultPlugin,
            FrameTimeDiagnosticsPlugin,
            ui::UiPlugin,
        ))
        .add_systems(PreStartup, systems::setup)
        .add_systems(PreUpdate, systems::debug_exit)
        .run();
}
