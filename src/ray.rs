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

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
