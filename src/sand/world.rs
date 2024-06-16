use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct SandWorld {
    grid: HashMap<IVec2, Entity>,
}

impl SandWorld {
    pub fn new() -> Self {
        Self {
            grid: HashMap::default(),
        }
    }

    pub fn get(&self, position: IVec2) -> Option<&Entity> {
        self.grid.get(&position)
    }

    pub fn insert(&mut self, position: IVec2, entity: Entity) {
        self.grid.insert(position, entity);
    }

    pub fn remove(&mut self, position: IVec2) -> Option<Entity> {
        self.grid.remove(&position)
    }
}
