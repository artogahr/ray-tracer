use std::sync::Arc;

use crate::{
    interval::Interval,
    material::*,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            } else {
                -*outward_normal
            }
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
            t: 0.0,
            front_face: true,
        }
    }
}
