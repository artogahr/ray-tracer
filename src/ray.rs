use crate::vec3::*;

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn ray_empty() {}

    pub fn ray(origin: Point3, direction: Vec3) {}

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
