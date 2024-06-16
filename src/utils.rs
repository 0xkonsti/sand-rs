use bevy::prelude::*;

pub trait VecParse {
    fn as_vec3(&self) -> Vec3;
    fn as_ivec2(&self) -> IVec2;
}

impl VecParse for IVec2 {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.0)
    }

    fn as_ivec2(&self) -> IVec2 {
        *self
    }
}

impl VecParse for Vec3 {
    fn as_vec3(&self) -> Vec3 {
        *self
    }

    fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x as i32, self.y as i32)
    }
}
