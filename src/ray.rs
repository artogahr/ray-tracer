use core::f64;

use crate::vec3::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn ray_empty() {}

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
