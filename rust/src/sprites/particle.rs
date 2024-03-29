use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::utils::{config::Config, general::random, vector::Vector};

use super::sprite::{CanvasDimension, Spritable, Sprite, SpriteData};

pub struct Particle {
    health: u32,
    pub is_faded: bool,
    pub sprite: Sprite,
    config: Config,
}

impl Spritable for Particle {
    fn update(&mut self) {
        self.sprite.update();

        if self.health < self.config.sprites.particle.minimal_health {
            self.is_faded = true;
        } else {
            self.health -= self.config.sprites.particle.health_decrement;
        }
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        if !self.is_faded {
            canvas.save();

            self.polygon(
                self.sprite.data.position,
                self.sprite.data.diameter,
                self.config.sprites.particle.polygon_sides,
                canvas.clone(),
            );

            canvas.restore();
        }
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        self.sprite.collide_with(sprite)
    }
}

impl Particle {
    pub fn new(sprite_data: SpriteData, canvas: CanvasDimension) -> Particle {
        let config = Config::new();

        let mut new_sprite_data = sprite_data;
        new_sprite_data.diameter = random(1, 5) as f64;
        new_sprite_data.velocity = Vector::random(-1.2, 1.2);

        Particle {
            health: random(175, 230),
            is_faded: false,
            sprite: Sprite::new(new_sprite_data, canvas),
            config,
        }
    }

    fn polygon(&self, position: Vector, radius: f64, sides: u8, canvas: CanvasRenderingContext2d) {
        let angle = 2.0 * PI / sides as f64;

        canvas.begin_path();

        canvas.move_to(position.x + radius, position.y);

        for side in 1..sides {
            let current_angle = angle * side as f64;
            let sx = position.x + current_angle.cos() * radius;
            let sy = position.y + current_angle.sin() * radius;

            canvas.line_to(sx, sy);
        }

        canvas.close_path();

        canvas.stroke();
    }
}
