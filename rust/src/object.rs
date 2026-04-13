use crate::bounds3::Bounds3;
use crate::intersection::Intersection;
use crate::ray::Ray;

pub trait Hittable: Send + Sync {
    fn get_intersection(&self, ray: &Ray) -> Intersection;
    fn get_bounds(&self) -> Bounds3;
    fn get_area(&self) -> f32;
    fn sample(&self, pos: &mut Intersection, pdf: &mut f32);
    fn has_emit(&self) -> bool;
}
