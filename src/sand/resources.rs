use super::grain::GrainType;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentGrainType(pub GrainType);

impl CurrentGrainType {
    pub fn create_grain(&self, commands: &mut Commands, position: IVec2) -> Entity {
        self.0.create(commands, position)
    }
}

impl Default for CurrentGrainType {
    fn default() -> Self {
        Self(GrainType::Sand)
    }
}

#[derive(Resource)]
pub struct SpawnDelay {
    timer: f32,
    delay: f32,
    guard: bool,
}

impl SpawnDelay {
    pub fn new(delay: f32) -> Self {
        Self {
            timer: 0.0,
            delay,
            guard: false,
        }
    }

    pub fn tick(&mut self, time: Res<Time>) {
        self.timer += time.delta_seconds();
        if self.timer >= self.delay {
            self.timer = 0.0;
            self.guard = false;
        }
    }

    pub fn consume(&mut self) -> bool {
        if !self.guard {
            self.timer = 0.0;
            self.guard = true;
            return true;
        }
        false
    }
}
