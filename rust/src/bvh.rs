use std::sync::Arc;

use crate::bounds3::{union_bounds, union_point, Bounds3};
use crate::global::get_random_float;
use crate::intersection::Intersection;
use crate::object::Hittable;
use crate::ray::Ray;

#[derive(Clone)]
pub struct BVHNode {
    pub bounds: Bounds3,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub object: Option<Arc<dyn Hittable>>,
    pub area: f32,
}

impl Default for BVHNode {
    fn default() -> Self {
        Self {
            bounds: Bounds3::default(),
            left: None,
            right: None,
            object: None,
            area: 0.0,
        }
    }
}

#[derive(Clone, Default)]
pub struct BVHAccel {
    pub root: Option<Box<BVHNode>>,
}

impl BVHAccel {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        if objects.is_empty() {
            return Self { root: None };
        }

        let root = Self::recursive_build(objects);
        Self {
            root: Some(Box::new(root)),
        }
    }

    fn recursive_build(mut objects: Vec<Arc<dyn Hittable>>) -> BVHNode {
        let mut node = BVHNode::default();

        let mut bounds = Bounds3::default();
        for obj in &objects {
            bounds = union_bounds(&bounds, &obj.get_bounds());
        }

        match objects.len() {
            1 => {
                let object = objects.remove(0);
                node.bounds = object.get_bounds();
                node.area = object.get_area();
                node.object = Some(object);
            }
            2 => {
                let right = objects.pop().expect("missing right object");
                let left = objects.pop().expect("missing left object");

                node.left = Some(Box::new(Self::recursive_build(vec![left])));
                node.right = Some(Box::new(Self::recursive_build(vec![right])));

                let left_ref = node.left.as_ref().expect("left child should exist");
                let right_ref = node.right.as_ref().expect("right child should exist");
                node.bounds = union_bounds(&left_ref.bounds, &right_ref.bounds);
                node.area = left_ref.area + right_ref.area;
            }
            _ => {
                let mut centroid_bounds = Bounds3::default();
                for obj in &objects {
                    centroid_bounds = union_point(&centroid_bounds, obj.get_bounds().centroid());
                }
                let dim = centroid_bounds.max_extent();

                objects.sort_by(|a, b| {
                    a.get_bounds()
                        .centroid()
                        .component(dim)
                        .partial_cmp(&b.get_bounds().centroid().component(dim))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let mid = objects.len() / 2;
                let right = objects.split_off(mid);
                let left = objects;

                node.left = Some(Box::new(Self::recursive_build(left)));
                node.right = Some(Box::new(Self::recursive_build(right)));

                let left_ref = node.left.as_ref().expect("left child should exist");
                let right_ref = node.right.as_ref().expect("right child should exist");
                node.bounds = union_bounds(&left_ref.bounds, &right_ref.bounds);
                node.area = left_ref.area + right_ref.area;
            }
        }

        node
    }

    pub fn intersect(&self, ray: &Ray) -> Intersection {
        match &self.root {
            Some(root) => Self::get_intersection(root, ray),
            None => Intersection::default(),
        }
    }

    fn get_intersection(node: &BVHNode, ray: &Ray) -> Intersection {
        let inv_dir = ray.direction_inv;
        let dir_is_neg = [
            (inv_dir.x < 0.0) as i32,
            (inv_dir.y < 0.0) as i32,
            (inv_dir.z < 0.0) as i32,
        ];

        if !node.bounds.intersect_p(ray, inv_dir, dir_is_neg) {
            return Intersection::default();
        }

        if node.left.is_none() && node.right.is_none() {
            if let Some(object) = &node.object {
                return object.get_intersection(ray);
            }
            return Intersection::default();
        }

        let hit_left = if let Some(left) = &node.left {
            Self::get_intersection(left, ray)
        } else {
            Intersection::default()
        };

        let hit_right = if let Some(right) = &node.right {
            Self::get_intersection(right, ray)
        } else {
            Intersection::default()
        };

        if hit_left.distance < hit_right.distance {
            hit_left
        } else {
            hit_right
        }
    }

    fn get_sample(node: &BVHNode, p: f32, pos: &mut Intersection, pdf: &mut f32) {
        if node.left.is_none() || node.right.is_none() {
            if let Some(object) = &node.object {
                object.sample(pos, pdf);
                *pdf *= node.area;
            }
            return;
        }

        let left = node.left.as_ref().expect("left child missing in sample");
        if p < left.area {
            Self::get_sample(left, p, pos, pdf);
        } else {
            let right = node.right.as_ref().expect("right child missing in sample");
            Self::get_sample(right, p - left.area, pos, pdf);
        }
    }

    pub fn sample(&self, pos: &mut Intersection, pdf: &mut f32) {
        if let Some(root) = &self.root {
            let p = get_random_float().sqrt() * root.area;
            Self::get_sample(root, p, pos, pdf);
            *pdf /= root.area;
        }
    }
}
