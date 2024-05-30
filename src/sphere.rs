use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::*,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let dir = r.direction();
        let a = dir.length_squared();
        let h = dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
