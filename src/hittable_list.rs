use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_from(object: impl Hittable + 'static) -> Self {
        HittableList {
            objects: vec![Rc::new(object) as Rc<dyn Hittable>],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
