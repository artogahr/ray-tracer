use core::f64;
use std::f32::consts::PI;

use crate::color::write_color;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::Ray;
use crate::vec3::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;

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
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = {
            if image_height < 1 {
                1
            } else {
                image_height
            }
        };
        // Calculate the vectors across the horizontal and down the vertical viewport edges. sg
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
        let w = (lookfrom - lookat).normalized();
        let u = (vup.cross(w)).normalized();
        let v = w.cross(u);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let center = lookfrom;
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_radius =
            focus_dist as f64 * f64::tan(degrees_to_radians(defocus_angle as f64 / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
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
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        // Render
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        for j in 0..self.image_height {
            bar.inc(self.image_width.into());
            let scanline: Vec<Color> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::default();
                    for _ in 0..self.samples_per_pixel {
                        let r: Ray = self.get_ray(i, j);
                        pixel_color += Self::ray_color(&r, self.max_depth, world);
                    }
                    self.pixel_samples_scale * pixel_color
                })
                .collect();
            for pixel in scanline {
                write_color(pixel);
            }
        }
        bar.finish();
    }

    pub fn ray_color(r: &Ray, depth: u32, world: &(impl Hittable + ?Sized)) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = HitRecord::default();

        if world.hit(r, Interval::from_values(0.001, f64::INFINITY), &mut rec) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * Camera::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
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
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI as f64 / 180.0
}
