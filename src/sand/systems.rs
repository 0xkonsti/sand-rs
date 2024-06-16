use super::events::{DespawnGrainEvent, SpawnGrainEvent};
use super::resources::CurrentGrainType;
use super::world::SandWorld;
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const FIXED_TIMESTEP: f64 = 0.016;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(SandWorld::new());
    commands.insert_resource(CurrentGrainType::default());
    commands.insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP));
}

pub fn spawn_grain(
    mut commands: Commands,
    mut events: EventReader<SpawnGrainEvent>,
    mut world: ResMut<SandWorld>,
    grain_type: Res<CurrentGrainType>,
) {
    for event in events.read() {
        if world.get(event.position).is_some() {
            continue;
        }
        let entity = grain_type.create_grain(&mut commands, event.position);
        world.insert(event.position, entity);
    }
}

pub fn despawn_grain(
    mut commands: Commands,
    mut events: EventReader<DespawnGrainEvent>,
    mut world: ResMut<SandWorld>,
) {
    for event in events.read() {
        if let Some(removed) = world.remove(event.position) {
            commands.entity(removed).despawn();
        }
    }
}

pub fn mouse_input(
    mut spawn_events: EventWriter<SpawnGrainEvent>,
    mut despawn_events: EventWriter<DespawnGrainEvent>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.any_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    if let Ok(window) = window_query.get_single() {
        if let Some(position) = window.cursor_position() {
            let position = IVec2::new(
                position.x as i32 / PIXELS_PER_UNIT as i32,
                (window.height() - position.y) as i32 / PIXELS_PER_UNIT as i32,
            );

            if mouse_button_input.pressed(MouseButton::Left) {
                spawn_events.send(SpawnGrainEvent::new(position));
            } else if mouse_button_input.pressed(MouseButton::Right) {
                despawn_events.send(DespawnGrainEvent::new(position));
            }
        }
    }
}
