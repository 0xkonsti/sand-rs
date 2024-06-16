use bevy::prelude::*;
use rand::prelude::*;

pub struct MovementOptionGroup(&'static [IVec2]);

impl MovementOptionGroup {
    pub fn shuffled(&self) -> Vec<IVec2> {
        let mut shuffled = self.0.to_vec();
        shuffled.shuffle(&mut thread_rng());
        shuffled
    }
}

pub type GMovement = &'static [MovementOptionGroup];

pub const SAND_MOVEMENT: GMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
];
