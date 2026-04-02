use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub direction_inv: Vec3,
    pub t: f32,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            direction_inv: Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z),
            t: 0.0,
            t_min: 0.0,
            t_max: f32::MAX,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
