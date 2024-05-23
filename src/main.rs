mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use std::rc::Rc;

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Point3};

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    let sun = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let land = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Rc::new(land));
    world.add(Rc::new(sun));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut cam: Camera = Camera::new(aspect_ratio, image_width, 100);

    cam.render(&world);
}
