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

    pub fn random() -> Vector {
        Vector {
            x: rand::thread_rng().gen_range(-1.2..1.2),
            y: rand::thread_rng().gen_range(-1.2..1.2),
        }
    }

    // pub fn set(&mut self, x: f64, y: f64) {
    //     self.x = x;
    //     self.y = y;
    // }
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
