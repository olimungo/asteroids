use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use super::sprite::{CanvasDimension, Spritable, Sprite, SpriteData};
use crate::{utils::random, vector::Vector};

const POLYGON_SIDES: u8 = 5;
const HEALTH_DECREMENT: u32 = 5;
const MINIMAL_HEALTH: u32 = 50;

pub struct Particle {
    health: u32,
    pub is_faded: bool,
    pub sprite: Sprite,
}

impl Spritable for Particle {
    fn update(&mut self) {
        self.sprite.update();

        if self.health < MINIMAL_HEALTH {
            self.is_faded = true;
        } else {
            self.health -= HEALTH_DECREMENT;
        }
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        if !self.is_faded {
            canvas.save();

            self.polygon(
                self.sprite.sprite_data.position,
                self.sprite.sprite_data.diameter,
                POLYGON_SIDES,
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
        let mut new_sprite_data = sprite_data;
        new_sprite_data.diameter = random(1, 4) as f64;
        new_sprite_data.velocity = Vector::random(-1.2, 1.2);

        Particle {
            health: random(175, 230),
            is_faded: false,
            sprite: Sprite::new(new_sprite_data, canvas),
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

        canvas.fill();
    }
}
