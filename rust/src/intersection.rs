use std::sync::Arc;

use crate::material::Material;
use crate::vector::Vec3;

#[derive(Clone)]
pub struct Intersection {
    pub happened: bool,
    pub coords: Vec3,
    pub tcoords: Vec3,
    pub normal: Vec3,
    pub emit: Vec3,
    pub distance: f32,
    pub material: Option<Arc<Material>>,
}

impl Default for Intersection {
    fn default() -> Self {
        Self {
            happened: false,
            coords: Vec3::zero(),
            tcoords: Vec3::zero(),
            normal: Vec3::zero(),
            emit: Vec3::zero(),
            distance: f32::MAX,
            material: None,
        }
    }
}
