use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Bounds3 {
    pub p_min: Vec3,
    pub p_max: Vec3,
}

impl Default for Bounds3 {
    fn default() -> Self {
        let min_num = f32::MIN;
        let max_num = f32::MAX;
        Self {
            p_max: Vec3::new(min_num, min_num, min_num),
            p_min: Vec3::new(max_num, max_num, max_num),
        }
    }
}

impl Bounds3 {
    pub fn from_points(p1: Vec3, p2: Vec3) -> Self {
        Self {
            p_min: Vec3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            p_max: Vec3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        }
    }

    pub fn diagonal(&self) -> Vec3 {
        self.p_max - self.p_min
    }

    pub fn max_extent(&self) -> usize {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn centroid(&self) -> Vec3 {
        0.5 * self.p_min + 0.5 * self.p_max
    }

    pub fn intersect_p(&self, ray: &Ray, inv_dir: Vec3, dir_is_neg: [i32; 3]) -> bool {
        let mut tx_min = (self.p_min.x - ray.origin.x) * inv_dir.x;
        let mut tx_max = (self.p_max.x - ray.origin.x) * inv_dir.x;
        if dir_is_neg[0] == 1 {
            std::mem::swap(&mut tx_min, &mut tx_max);
        }

        let mut ty_min = (self.p_min.y - ray.origin.y) * inv_dir.y;
        let mut ty_max = (self.p_max.y - ray.origin.y) * inv_dir.y;
        if dir_is_neg[1] == 1 {
            std::mem::swap(&mut ty_min, &mut ty_max);
        }

        let mut t_min = tx_min.max(ty_min);
        let mut t_max = tx_max.min(ty_max);
        if t_max < t_min {
            return false;
        }

        let mut tz_min = (self.p_min.z - ray.origin.z) * inv_dir.z;
        let mut tz_max = (self.p_max.z - ray.origin.z) * inv_dir.z;
        if dir_is_neg[2] == 1 {
            std::mem::swap(&mut tz_min, &mut tz_max);
        }

        t_min = t_min.max(tz_min);
        t_max = t_max.min(tz_max);
        if t_max < t_min {
            return false;
        }

        t_max >= 0.0 && t_min <= t_max
    }
}

pub fn union_bounds(b1: &Bounds3, b2: &Bounds3) -> Bounds3 {
    Bounds3 {
        p_min: Vec3::min(b1.p_min, b2.p_min),
        p_max: Vec3::max(b1.p_max, b2.p_max),
    }
}

pub fn union_point(b: &Bounds3, p: Vec3) -> Bounds3 {
    Bounds3 {
        p_min: Vec3::min(b.p_min, p),
        p_max: Vec3::max(b.p_max, p),
    }
}
