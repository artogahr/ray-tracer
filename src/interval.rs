use core::f64;
#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn from_values(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
