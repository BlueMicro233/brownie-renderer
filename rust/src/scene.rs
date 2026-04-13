use std::sync::Arc;

use crate::bvh::BVHAccel;
use crate::global::{get_random_float, M_PI};
use crate::intersection::Intersection;
use crate::object::Hittable;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub background_color: Vec3,
    pub max_depth: i32,
    pub russian_roulette: f32,
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bvh: Option<BVHAccel>,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fov: 40.0,
            background_color: Vec3::new(0.235294, 0.67451, 0.843137),
            max_depth: 1,
            russian_roulette: 0.8,
            objects: Vec::new(),
            bvh: None,
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn build_bvh(&mut self) {
        println!(" - Generating BVH...\n");
        self.bvh = Some(BVHAccel::new(self.objects.clone()));
    }

    pub fn intersect(&self, ray: &Ray) -> Intersection {
        if let Some(bvh) = &self.bvh {
            bvh.intersect(ray)
        } else {
            Intersection::default()
        }
    }

    pub fn sample_light(&self, pos: &mut Intersection, pdf: &mut f32) {
        let mut emit_area_sum = 0.0;
        for object in &self.objects {
            if object.has_emit() {
                emit_area_sum += object.get_area();
            }
        }

        let p = get_random_float() * emit_area_sum;
        emit_area_sum = 0.0;

        for object in &self.objects {
            if object.has_emit() {
                emit_area_sum += object.get_area();
                if p <= emit_area_sum {
                    object.sample(pos, pdf);
                    break;
                }
            }
        }
    }

    pub fn cast_ray(&self, ray: &Ray, depth: i32) -> Vec3 {
        let p = self.intersect(ray);
        if !p.happened {
            return Vec3::zero();
        }

        let material = p.material.as_ref().expect("material missing at hit point");

        if material.has_emission() {
            if depth == 0 {
                return material.get_emission();
            }
            return Vec3::zero();
        }

        let wo = (ray.origin - p.coords).normalize();

        let mut x = Intersection::default();
        let mut pdf_area_light = 0.0f32;
        self.sample_light(&mut x, &mut pdf_area_light);

        let ws = (x.coords - p.coords).normalize();
        let shadow_ray = Ray::new(p.coords + ws * 0.0005, ws);
        let is_block = self.intersect(&shadow_ray);

        let l_dir = if is_block.happened && (is_block.coords - x.coords).norm() < 0.001 {
            x.emit * material.eval(wo, ws, p.normal) * ws.dot(p.normal) * (-ws).dot(x.normal)
                / (x.coords - p.coords).norm().powi(2)
                / pdf_area_light
        } else {
            Vec3::zero()
        };

        if depth > 4 && get_random_float() > self.russian_roulette {
            return l_dir;
        }

        let wi = material.sample(wo, p.normal);
        let cos_theta = wi.dot(p.normal);
        let pdf_indir = cos_theta / M_PI;

        let l_indir = if pdf_indir > 0.0 && cos_theta > 0.0 {
            let shading = self.cast_ray(&Ray::new(p.coords + wi * 0.0005, wi), depth + 1);
            let rr = if depth > 4 {
                self.russian_roulette
            } else {
                1.0
            };
            shading * material.eval(wo, wi, p.normal) * cos_theta / pdf_indir / rr
        } else {
            Vec3::zero()
        };

        l_dir + l_indir
    }
}
