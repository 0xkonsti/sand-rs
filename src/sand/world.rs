use super::grain::GrainType;
use crate::utils::{VecOrder, VecParse};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct SandWorld {
    grid: HashMap<IVec2, Entity>,
    sorted: Vec<IVec2>,
}

impl SandWorld {
    pub fn new() -> Self {
        Self {
            grid: HashMap::default(),
            sorted: Vec::new(),
        }
    }

    pub fn get(&self, position: &IVec2) -> Option<Entity> {
        self.grid.get(position).copied()
    }

    pub fn insert(&mut self, position: IVec2, entity: Entity) {
        self.grid.insert(position, entity);
        match self.sorted.binary_search_by(|&x| x.vec_cmp(&position)) {
            Ok(_) => {}
            Err(index) => self.sorted.insert(index, position),
        }
    }

    pub fn remove(&mut self, position: &IVec2) -> Option<Entity> {
        if let Some(removed) = self.grid.remove(position) {
            self.sorted.retain(|x| x != position);
            Some(removed)
        } else {
            None
        }
    }

    pub fn move_entity(&mut self, from: &IVec2, to: IVec2) {
        if let Some(entity) = self.remove(from) {
            self.insert(to, entity);
        }
    }

    pub fn update(&mut self, query: &mut Query<(&GrainType, &mut Transform)>) {
        let mut new_grid = self.grid.clone();
        let mut updated = Vec::new();

        for position in self.sorted.iter().copied() {
            if let Some(entity) = self.get(&position) {
                if let Ok((grain_type, mut transform)) = query.get_mut(entity) {
                    if let Some(new_position) =
                        grain_type.update(position, |position| new_grid.get(position).copied())
                    {
                        new_grid.remove(&position);
                        new_grid.insert(new_position, entity);
                        transform.translation = new_position.as_vec3();

                        updated.push((position, new_position));
                    }
                }
            }
        }

        for (from, to) in updated {
            self.move_entity(&from, to);
        }
    }
}
