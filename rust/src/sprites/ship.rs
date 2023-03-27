use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{log, vector::Vector};

use super::{
    laser::Laser,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

pub struct Ship {
    sprite: Sprite,
    heading: f64,
    is_boosting: bool,
    lasers: Vec<Laser>,
}

impl Spritable for Ship {
    fn update(&mut self) {
        self.sprite.update();

        self.heading += self.sprite.sprite_data.rotation;

        if self.is_boosting {
            let mut force = Vector::from_angle(self.heading);
            force.limit(0.15);
            self.sprite.sprite_data.velocity += force;
        }

        self.sprite.sprite_data.velocity.limit(10.0);
        self.sprite.sprite_data.velocity.mult(0.995);

        for laser in &mut self.lasers {
            laser.update();
        }

        self.lasers = self
            .lasers
            .drain(..)
            .filter(|laser| !laser.sprite.is_offscreen)
            .collect();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let position = self.sprite.sprite_data.position;
        let diameter = self.sprite.sprite_data.diameter;
        let _result = canvas.translate(position.x, position.y);
        let _result = canvas.rotate(self.heading + PI / 2.0);
        let smaller_radius = (diameter / 10.0) * 3.5;

        canvas.begin_path();

        canvas.move_to(smaller_radius, diameter / 2.0);
        canvas.line_to(0.0, -diameter / 2.0);
        canvas.line_to(-smaller_radius, diameter / 2.0);
        canvas.bezier_curve_to(
            0.0,
            smaller_radius,
            0.0,
            smaller_radius,
            smaller_radius,
            diameter / 2.0,
        );

        canvas.close_path();

        canvas.stroke();
        canvas.restore();

        for laser in &self.lasers {
            laser.draw(canvas.clone());
        }
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        false
    }
}

impl Ship {
    pub fn new(sprite_data: SpriteData, canvas: CanvasDimension) -> Ship {
        Ship {
            sprite: Sprite::new(sprite_data, canvas),
            heading: -PI / 2.0,
            is_boosting: false,
            lasers: Vec::new(),
        }
    }

    pub fn set_boost(&mut self, value: bool) {
        self.is_boosting = value;
    }

    pub fn set_rotation(&mut self, angle: f64) {
        self.sprite.sprite_data.rotation = angle;
    }

    pub fn shoot(&mut self) {
        let position = self.sprite.sprite_data.position;
        let mut velocity = Vector::from_angle(self.heading);
        velocity.mult(10.0);

        let diameter = 2.0;
        let rotation = 0.0;
        let rotation_step = 0.0;

        let laser = Laser::new(
            SpriteData {
                position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            },
            self.sprite.canvas,
        );

        self.lasers.push(laser);
    }

    pub fn lasers_collide_with(&mut self, sprite: Sprite) -> bool {
        for (index, laser) in &mut self.lasers.iter().enumerate() {
            if laser.collide_with(sprite) {
                self.lasers.remove(index);
                return true;
            }
        }

        false
    }
}
