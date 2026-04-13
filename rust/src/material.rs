use crate::global::{get_random_float, EPSILON, M_PI};
use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    Diffuse,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub m_type: MaterialType,
    pub emission: Vec3,
    pub ior: f32,
    pub kd: Vec3,
    pub ks: Vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub fn new(t: MaterialType, emission: Vec3) -> Self {
        Self {
            m_type: t,
            emission,
            ior: 1.5,
            kd: Vec3::zero(),
            ks: Vec3::zero(),
            specular_exponent: 0.0,
        }
    }

    pub fn has_emission(&self) -> bool {
        self.emission.norm() > EPSILON
    }

    pub fn get_emission(&self) -> Vec3 {
        self.emission
    }

    fn to_world(a: Vec3, n: Vec3) -> Vec3 {
        let c = if n.x.abs() > n.y.abs() {
            let inv_len = 1.0 / (n.x * n.x + n.z * n.z).sqrt();
            Vec3::new(n.z * inv_len, 0.0, -n.x * inv_len)
        } else {
            let inv_len = 1.0 / (n.y * n.y + n.z * n.z).sqrt();
            Vec3::new(0.0, n.z * inv_len, -n.y * inv_len)
        };
        let b = c.cross(n);
        a.x * b + a.y * c + a.z * n
    }

    pub fn sample(&self, _wi: Vec3, n: Vec3) -> Vec3 {
        match self.m_type {
            MaterialType::Diffuse => {
                let r1 = get_random_float();
                let r2 = get_random_float();

                let phi = 2.0 * M_PI * r1;
                let r = r2.sqrt();

                let x = r * phi.cos();
                let y = r * phi.sin();
                let z = (1.0 - r2).sqrt();

                let local_ray = Vec3::new(x, y, z);
                Self::to_world(local_ray, n)
            }
        }
    }

    pub fn pdf(&self, _wi: Vec3, wo: Vec3, n: Vec3) -> f32 {
        match self.m_type {
            MaterialType::Diffuse => {
                if wo.dot(n) > 0.0 {
                    0.5 / M_PI
                } else {
                    0.0
                }
            }
        }
    }

    pub fn eval(&self, _wi: Vec3, wo: Vec3, n: Vec3) -> Vec3 {
        match self.m_type {
            MaterialType::Diffuse => {
                let cosalpha = n.dot(wo);
                if cosalpha > 0.0 {
                    self.kd / M_PI
                } else {
                    Vec3::zero()
                }
            }
        }
    }
}
