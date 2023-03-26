use rand::Rng;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn random(max_x: u32, max_y: u32) -> Vector {
        Vector {
            x: rand::thread_rng().gen_range(0.0..max_x as f64),
            y: rand::thread_rng().gen_range(0.0..max_y as f64),
        }
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
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
