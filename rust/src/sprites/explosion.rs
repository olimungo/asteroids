use web_sys::CanvasRenderingContext2d;

use super::{
    particle::Particle,
    sprite::{CanvasDimension, Spritable, SpriteData},
};

const COUNT_PARTICULES: u8 = 8;

pub struct Explosion {
    particles: Vec<Particle>,
    pub is_faded: bool,
}

impl Explosion {
    pub fn new(sprite_data: SpriteData, canvas: CanvasDimension) -> Explosion {
        let mut particles = Vec::new();

        for _count in 0..COUNT_PARTICULES {
            particles.push(Particle::new(sprite_data, canvas));
        }

        Explosion {
            particles,
            is_faded: false,
        }
    }
    pub fn update(&mut self) {
        for index in (0..self.particles.len()).rev() {
            self.particles[index].update();

            if self.particles[index].is_faded {
                self.particles.remove(index);
            }
        }

        if self.particles.is_empty() {
            self.is_faded = true;
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        for particle in &self.particles {
            particle.draw(canvas.clone());
        }
    }
}
