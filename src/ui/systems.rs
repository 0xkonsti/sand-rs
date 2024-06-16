use super::ui_components::fps_display;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    fps_display::setup(&mut commands);
}
