use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::{hit_sphere, Ray},
    vec3::*,
};

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    println!("{rbyte} {gbyte} {bbyte}")
}

pub fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, Interval::from_values(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = r.direction().normalized();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
