use crate::utils::VecParse;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::*;

mod color;
mod movement;

const GRAIN_SIZE: Vec2 = Vec2::new(1.0, 1.0);

#[derive(Component, Debug, Clone, Copy)]
pub enum GrainType {
    Sand,
    Water,
}

impl GrainType {
    pub fn create(self, commands: &mut Commands, position: IVec2) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: *self.color().unwrap_or(color::DEFAULT_COLOR),
                        custom_size: Some(GRAIN_SIZE),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(position.as_vec3()),
                    ..default()
                },
                self,
            ))
            .id()
    }

    pub fn color(&self) -> Option<&Color> {
        match self {
            GrainType::Sand => color::SAND_COLOR,
            GrainType::Water => color::WATER_COLOR,
        }
        .choose(&mut thread_rng())
    }

    fn movement(&self) -> movement::GMovement {
        match self {
            GrainType::Sand => movement::SAND_MOVEMENT,
            GrainType::Water => movement::WATER_MOVEMENT,
        }
    }

    pub fn update<F>(&self, position: IVec2, lookup: F) -> Option<IVec2>
    where
        F: Fn(&IVec2) -> Option<Entity>,
    {
        for group in self.movement() {
            for direction in group.shuffled() {
                let new_position = position + direction;
                if new_position.y < 0 {
                    continue;
                }
                if lookup(&new_position).is_none() {
                    return Some(new_position);
                }
            }
        }

        None
    }
}
