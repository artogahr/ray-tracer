use core::f64;
use std::default;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }

    pub fn from_values(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Interval = Interval {
        min: std::f64::INFINITY,
        max: std::f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: std::f64::NEG_INFINITY,
        max: std::f64::INFINITY,
    };
}
