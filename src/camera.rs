use crate::color::write_color;
use crate::hittable::*;
use crate::interval::*;
use crate::ray::Ray;
use crate::vec3::*;
use indicatif::ProgressBar;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
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
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        // Render

        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray {
                    orig: self.center,
                    dir: ray_direction,
                };
                let pixel_color = Self::ray_color(&r, world);
                write_color(pixel_color);
            }
        }
        bar.finish();
    }

    fn ray_color(r: &Ray, world: &(impl Hittable + ?Sized)) -> Color {
        let mut rec: HitRecord = HitRecord::default();

        if world.hit(r, Interval::from_values(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction: Vec3 = r.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
