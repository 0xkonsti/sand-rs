use bevy::prelude::*;

pub trait VecParse {
    fn as_vec3(&self) -> Vec3;
}

impl VecParse for IVec2 {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.0)
    }
}
