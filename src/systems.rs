use crate::components::MainCamera;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};

pub const PIXELS_PER_UNIT: f32 = 16.0;

const CLEAR_COLOR: Color = Color::rgb(0.035, 0., 0.070);
const RESOLUTION: (f32, f32) = (1280., 720.);

pub fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    window.title = "Falling Sand".into();
    window.present_mode = PresentMode::AutoNoVsync;
    window.mode = WindowMode::Windowed;
    window.resolution = RESOLUTION.into();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / (2.0 * PIXELS_PER_UNIT),
                window.height() / (2.0 * PIXELS_PER_UNIT),
                999.0,
            ),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
                ..Default::default()
            },
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize(PIXELS_PER_UNIT),
                ..default()
            },
            ..Default::default()
        },
        MainCamera,
    ));
}

pub fn debug_exit(mut exit: EventWriter<bevy::app::AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        debug!("ESC pressed, exiting...");
        exit.send(bevy::app::AppExit);
    }
}
