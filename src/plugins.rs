use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

const LOG_LEVEL: Level = Level::INFO;

pub struct CustomDefaultPlugin;

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: LOG_LEVEL,
                    ..default()
                }),
        );
    }
}
