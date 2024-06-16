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
