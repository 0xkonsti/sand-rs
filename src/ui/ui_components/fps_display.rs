use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const FPS_COLOR: Color = Color::rgb(0.949, 0.706, 0.51);

#[derive(Component)]
pub struct FpsDisplay;

pub fn setup(commands: &mut Commands) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font_size: 13.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 13.,
                    color: FPS_COLOR,
                    ..default()
                }),
            ])
            .with_justify(JustifyText::Left),
            z_index: ZIndex::Global(999), // Always on top
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(8.),
            left: Val::Px(8.),
            ..default()
        }),
        FpsDisplay,
    ));
}

pub fn update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsDisplay>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
