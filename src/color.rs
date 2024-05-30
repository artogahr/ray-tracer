use crate::{interval::Interval, vec3::*};

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    let gamma = 2.0;
    if linear_component > 0.0 {
        linear_component.powf(1.0 / gamma)
    } else {
        0.0
    }
}

#[inline]
pub fn write_color(pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity: Interval = Interval::from_values(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u32;
    let gbyte = (256.0 * intensity.clamp(g)) as u32;
    let bbyte = (256.0 * intensity.clamp(b)) as u32;

    println!("{rbyte} {gbyte} {bbyte}")
}
