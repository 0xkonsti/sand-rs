use super::grain::GrainType;
use crate::utils::VecParse;
use bevy::prelude::*;
use bevy::utils::HashMap;

const GROUND_LEVEL: i32 = 0;

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

    pub fn get(&self, position: IVec2) -> Option<Entity> {
        self.grid.get(&position).copied()
    }

    pub fn insert(&mut self, position: IVec2, entity: Entity) {
        self.grid.insert(position, entity);
    }

    pub fn remove(&mut self, position: IVec2) -> Option<Entity> {
        self.grid.remove(&position)
    }

    pub fn move_entity(&mut self, from: IVec2, to: IVec2) {
        if let Some(entity) = self.remove(from) {
            self.insert(to, entity);
        }
    }

    pub fn update(&mut self, query: &mut Query<(&GrainType, &mut Transform)>) {
        for (grain_type, mut transform) in query.iter_mut() {
            let position = transform.translation.as_ivec2();
            if let Some(new_position) = grain_type.update(position, |position| self.get(position)) {
                if new_position.y < GROUND_LEVEL {
                    continue;
                }
                self.move_entity(position, new_position);
                transform.translation = new_position.as_vec3();
            }
        }
    }
}
