use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn random(min_limit: f64, max_limit: f64) -> Vector {
        Vector {
            x: rand::thread_rng().gen_range(min_limit..max_limit),
            y: rand::thread_rng().gen_range(min_limit..max_limit),
        }
    }

    pub fn random_velocity(limit: f64, min_value: f64) -> Vector {
        let constrain_limit = limit - min_value;
        let mut x = rand::thread_rng().gen_range(-constrain_limit..constrain_limit);
        let mut y = rand::thread_rng().gen_range(-constrain_limit..constrain_limit);

        if x > 0.0 {
            x += min_value;
        } else {
            x -= min_value;
        }

        if y > 0.0 {
            y += min_value;
        } else {
            y -= min_value;
        }

        Vector { x, y }
    }

    pub fn from_angle(angle: f64) -> Vector {
        Vector::new(angle.cos(), angle.sin())
    }

    pub fn limit(&mut self, max: f64) {
        let squared_magnetitude = self.squared_magnetitude();
        let squared_max = max * max;

        if squared_magnetitude > squared_max {
            self.div(squared_magnetitude.sqrt());
            self.mult(max);
        }
    }

    pub fn squared_magnetitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn div(&mut self, value: f64) -> Self {
        self.x /= value;
        self.y /= value;

        *self
    }

    pub fn mult(&mut self, value: f64) -> Self {
        self.x *= value;
        self.y *= value;

        *self
    }

    pub fn distance(self, v: Vector) -> f64 {
        let x = v.x - self.x;
        let y = v.y - self.y;
        (x * x + y * y).sqrt()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}
