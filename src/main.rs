mod color;
mod ray;
mod vec3;
use indicatif::ProgressBar;

use crate::{
    color::*,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    //Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    let bar = ProgressBar::new((image_width * image_height) as u64);

    for j in 0..image_height {
        for i in 0..image_width {
            bar.inc(1);
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray {
                orig: pixel_center,
                dir: ray_direction,
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    bar.finish();
}
