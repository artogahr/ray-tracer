use crate::vec3::*;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    println!("{rbyte} {gbyte} {bbyte}")
}
