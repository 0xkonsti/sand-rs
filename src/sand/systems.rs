use super::events::{DespawnGrainEvent, SpawnGrainEvent};
use super::grain::GrainType;
use super::resources::{CurrentGrainType, SpawnDelay};
use super::world::SandWorld;
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const FIXED_TIMESTEP: f64 = 0.016;
const SPAWN_DELAY: f32 = 0.04;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(SandWorld::new());
    commands.insert_resource(CurrentGrainType::default());
    commands.insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP));
    commands.insert_resource(SpawnDelay::new(SPAWN_DELAY));
}

pub fn update(mut spawn_delay: ResMut<SpawnDelay>, time: Res<Time>) {
    spawn_delay.tick(time);
}

pub fn fixed_update(mut world: ResMut<SandWorld>, mut query: Query<(&GrainType, &mut Transform)>) {
    world.update(&mut query);
}

pub fn spawn_grain(
    mut commands: Commands,
    mut events: EventReader<SpawnGrainEvent>,
    mut world: ResMut<SandWorld>,
    mut spawn_delay: ResMut<SpawnDelay>,
    grain_type: Res<CurrentGrainType>,
) {
    if !spawn_delay.consume() {
        return;
    }

    if let Some(first) = events.read().next() {
        if world.get(first.position).is_some() {
            return;
        }
        let entity = grain_type.create_grain(&mut commands, first.position);
        world.insert(first.position, entity);
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
