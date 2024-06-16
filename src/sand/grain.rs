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
        }
        .choose(&mut thread_rng())
    }

    pub fn movement(&self) -> Option<movement::GMovement> {
        Some(match self {
            GrainType::Sand => movement::SAND_MOVEMENT,
        })
    }
}
