use core::f64;

use crate::color::write_color;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::Ray;
use crate::vec3::*;
use indicatif::ProgressBar;
use rand::thread_rng;
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = {
            if image_height < 1 {
                1
            } else {
                image_height
            }
        };
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let center = Point3::new(0.0, 0.0, 0.0);
        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        // Render

        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, world);
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        bar.finish();
    }

    fn ray_color(r: &Ray, world: &(impl Hittable + ?Sized)) -> Color {
        let mut rec: HitRecord = HitRecord::default();

        if world.hit(r, Interval::from_values(0.0, f64::INFINITY), &mut rec) {
            let direction: Vec3 = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * Self::ray_color(&Ray::new(rec.p, direction), world);
        }

        let unit_direction: Vec3 = r.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            thread_rng().gen_range(0.0..0.999) - 0.5,
            thread_rng().gen_range(0.0..0.999) - 0.5,
            0.0,
        )
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}