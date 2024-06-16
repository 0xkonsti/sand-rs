use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SpawnGrainEvent {
    pub position: IVec2,
}

impl SpawnGrainEvent {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
        }
    }
}

#[derive(Event, Debug)]
pub struct DespawnGrainEvent {
    pub position: IVec2,
}

impl DespawnGrainEvent {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
        }
    }
}
