use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, vector::Vector};

use super::{
    laser::Laser,
    potatoid::Potatoid,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

pub struct Ship {
    pub sprite: Sprite,
    pub heading: f64,
    is_boosting: bool,
    is_plain_ship: bool,
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
        for laser in &self.lasers {
            laser.draw(canvas.clone());
        }

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

        if self.is_plain_ship {
            canvas.set_fill_style(&Colors::Edge.value().into());
            canvas.fill();
        } else {
            canvas.set_fill_style(&Colors::Background.value().into());
            canvas.fill();
            canvas.stroke();
        }

        canvas.restore();
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        self.sprite.collide_with(sprite)
    }
}

impl Ship {
    pub fn new(sprite_data: SpriteData, is_plain_ship: bool, canvas: CanvasDimension) -> Ship {
        Ship {
            sprite: Sprite::new(sprite_data, canvas),
            heading: -PI / 2.0,
            is_boosting: false,
            is_plain_ship,
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

    pub fn break_up(&self) -> Vec<Potatoid> {
        let mut new_potatoids = Vec::new();
        let potatoids_data = [
            [0.9, 0.05, 5.0],
            [0.9, -0.08, 6.0],
            [0.9, 0.15, 3.0],
            [0.7, -0.1, 5.0],
        ];

        for potatoid_data in potatoids_data {
            let mut sprite_data = self.sprite.sprite_data;

            sprite_data.diameter *= potatoid_data[0];
            sprite_data.velocity = Vector::random(-1.2, 1.2);
            sprite_data.rotation_step = potatoid_data[1];

            new_potatoids.push(Potatoid::new(
                sprite_data,
                potatoid_data[2] as u8,
                self.sprite.canvas,
            ));
        }

        new_potatoids
    }
}
