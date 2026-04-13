use std::path::Path;
use std::sync::Arc;

use crate::bounds3::Bounds3;
use crate::bvh::BVHAccel;
use crate::global::{get_random_float, EPSILON};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::object::Hittable;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub normal: Vec3,
    pub area: f32,
    pub material: Arc<Material>,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<Material>) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2).normalize();
        let area = e1.cross(e2).norm() * 0.5;
        Self {
            v0,
            v1,
            v2,
            normal,
            area,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn get_intersection(&self, ray: &Ray) -> Intersection {
        let mut intersection = Intersection::default();

        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let s = ray.origin - self.v0;
        let s1 = ray.direction.cross(edge2);
        let s2 = s.cross(edge1);

        let s1_dot_e1 = s1.dot(edge1);
        if s1_dot_e1 < EPSILON {
            return intersection;
        }
        let inv = 1.0 / s1_dot_e1;

        let t = inv * s2.dot(edge2);
        let u = inv * s1.dot(s);
        let v = inv * s2.dot(ray.direction);

        if t > 0.0 && (1.0 - u - v) > 0.0 && u > 0.0 && v > 0.0 {
            intersection.happened = true;
            intersection.coords = ray.origin + ray.direction * t;
            intersection.normal = self.normal;
            intersection.distance = t;
            intersection.material = Some(self.material.clone());
        }

        intersection
    }

    fn get_bounds(&self) -> Bounds3 {
        let b = Bounds3::from_points(self.v0, self.v1);
        Bounds3 {
            p_min: Vec3::min(b.p_min, self.v2),
            p_max: Vec3::max(b.p_max, self.v2),
        }
    }

    fn get_area(&self) -> f32 {
        self.area
    }

    fn sample(&self, pos: &mut Intersection, pdf: &mut f32) {
        let x = get_random_float().sqrt();
        let y = get_random_float();

        pos.coords = self.v0 * (1.0 - x) + self.v1 * (x * (1.0 - y)) + self.v2 * (x * y);
        pos.normal = self.normal;
        pos.material = Some(self.material.clone());
        *pdf = 1.0 / self.area;
    }

    fn has_emit(&self) -> bool {
        self.material.has_emission()
    }
}

pub struct MeshTriangle {
    pub triangles: Vec<Arc<dyn Hittable>>,
    pub bvh: BVHAccel,
    pub bounding_box: Bounds3,
    pub area: f32,
    pub material: Arc<Material>,
}

impl MeshTriangle {
    pub fn new<P: AsRef<Path>>(filename: P, material: Arc<Material>) -> Result<Self, String> {
        let options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };

        let (models, _) = tobj::load_obj(filename.as_ref(), &options)
            .map_err(|e| format!("failed to load obj {}: {e}", filename.as_ref().display()))?;

        let mut triangles: Vec<Arc<dyn Hittable>> = Vec::new();
        let mut area = 0.0f32;

        let mut min_vert = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max_vert = Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

        for model in models {
            let mesh = model.mesh;
            for idx in mesh.indices.chunks_exact(3) {
                let i0 = idx[0] as usize;
                let i1 = idx[1] as usize;
                let i2 = idx[2] as usize;

                let v0 = Vec3::new(
                    mesh.positions[i0 * 3],
                    mesh.positions[i0 * 3 + 1],
                    mesh.positions[i0 * 3 + 2],
                );
                let v1 = Vec3::new(
                    mesh.positions[i1 * 3],
                    mesh.positions[i1 * 3 + 1],
                    mesh.positions[i1 * 3 + 2],
                );
                let v2 = Vec3::new(
                    mesh.positions[i2 * 3],
                    mesh.positions[i2 * 3 + 1],
                    mesh.positions[i2 * 3 + 2],
                );

                min_vert = Vec3::min(min_vert, v0);
                min_vert = Vec3::min(min_vert, v1);
                min_vert = Vec3::min(min_vert, v2);
                max_vert = Vec3::max(max_vert, v0);
                max_vert = Vec3::max(max_vert, v1);
                max_vert = Vec3::max(max_vert, v2);

                let tri = Arc::new(Triangle::new(v0, v1, v2, material.clone()));
                area += tri.area;
                triangles.push(tri);
            }
        }

        if triangles.is_empty() {
            return Err(format!(
                "no triangles loaded from {}",
                filename.as_ref().display()
            ));
        }

        let bounding_box = Bounds3::from_points(min_vert, max_vert);

        let bvh = BVHAccel::new(triangles.clone());

        Ok(Self {
            triangles,
            bvh,
            bounding_box,
            area,
            material,
        })
    }
}

impl Hittable for MeshTriangle {
    fn get_intersection(&self, ray: &Ray) -> Intersection {
        self.bvh.intersect(ray)
    }

    fn get_bounds(&self) -> Bounds3 {
        self.bounding_box
    }

    fn get_area(&self) -> f32 {
        self.area
    }

    fn sample(&self, pos: &mut Intersection, pdf: &mut f32) {
        self.bvh.sample(pos, pdf);
        pos.emit = self.material.get_emission();
    }

    fn has_emit(&self) -> bool {
        self.material.has_emission()
    }
}
