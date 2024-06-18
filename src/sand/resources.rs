use super::grain::GrainType;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentGrainType(pub GrainType);

impl CurrentGrainType {
    pub fn create_grain(&self, commands: &mut Commands, position: IVec2) -> Entity {
        self.0.create(commands, position)
    }

    pub fn set(&mut self, grain_type: GrainType) {
        self.0 = grain_type;
    }

    pub fn unlimited(&self) -> bool {
        self.0 == GrainType::Stone || self.0 == GrainType::Water
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

type GBrush = &'static [IVec2];

const BRUSH_DOT: GBrush = &[IVec2::new(0, 0)];
const BRUSH_MEDIUM: GBrush = &[
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(-2, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
];

#[derive(Resource)]
pub struct Brush {
    current: usize,
    brushes: Vec<GBrush>,
}

impl Brush {
    pub fn current(&self) -> GBrush {
        self.brushes[self.current]
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.brushes.len();
    }

    pub fn previous(&mut self) {
        self.current = (self.current + self.brushes.len() - 1) % self.brushes.len();
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            current: 1,
            brushes: vec![BRUSH_DOT, BRUSH_MEDIUM],
        }
    }
}
