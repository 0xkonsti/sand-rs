mod events;
mod grain;
mod resources;
mod systems;
mod world;

use bevy::prelude::*;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnGrainEvent>()
            .add_event::<events::DespawnGrainEvent>()
            .add_systems(Startup, systems::setup)
            .add_systems(
                PreUpdate,
                (
                    systems::mouse_input,
                    (systems::spawn_grain, systems::despawn_grain).chain(),
                ),
            )
            .add_systems(Update, systems::update)
            .add_systems(FixedUpdate, systems::fixed_update);
    }
}
