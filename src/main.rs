mod color;
mod ray;
mod vec3;
use indicatif::ProgressBar;

use crate::{color::write_color, vec3::Color};

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    println!("P3\n{image_width} {image_height}\n255");

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));

    // let bar = ProgressBar::new((image_width * image_height) as u64);
    //
    // for j in 0..image_height {
    //     bar.inc(1);
    //     for i in 0..image_width {
    //         let pixel_color = Color::new(
    //             (i as f64) / (image_width - 1) as f64,
    //             (j as f64) / (image_height - 1) as f64,
    //             0.0,
    //         );
    //         write_color(pixel_color);
    //     }
    // }
    // bar.finish();
}
