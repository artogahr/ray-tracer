use core::f64;
use std::fmt;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use rand::{thread_rng, Rng};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn clear(&mut self) {
        self.e[0] = 0.0;
        self.e[1] = 0.0;
        self.e[2] = 0.0;
    }

    pub fn random() -> Self {
        Self::new(thread_rng().gen(), thread_rng().gen(), thread_rng().gen())
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self::new(
            thread_rng().gen_range(min..max),
            thread_rng().gen_range(min..max),
            thread_rng().gen_range(min..max),
        )
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere: Vec3 = Self::random_unit_vector();
        if on_unit_sphere.dot(*normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn format_color(self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self[0]) as u64,
            (255.999 * self[1]) as u64,
            (255.999 * self[2]) as u64
        )
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1.0e-8;
        if self[0].abs() < s && self[1].abs() < s && self[2].abs() < s {
            return true;
        }
        false
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(n.dot(-*uv), 1.0);
        let r_out_perp: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel: Vec3 = (-1.0 * f64::abs(1.0 - r_out_perp.length_squared()).sqrt()) * *n;
        r_out_perp + r_out_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs[0] + self, rhs[1] + self, rhs[2] + self],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other[0], self * other[1], self * other[2]],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Vec3 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        };
    }
}
