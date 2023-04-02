use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, interval::Interval, vector::Vector};

use super::{
    laser::Laser,
    potatoid::Potatoid,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

const SHIP_SHELL_SIZE: f64 = 36.0;
const BOOSTER_INTERVAL: u32 = 150;

pub struct Ship {
    pub sprite: Sprite,
    pub heading: f64,
    is_boosting: bool,
    is_plain_ship: bool,
    booster_flames_interval: Interval,
    switch_flames: bool,
    lasers: Vec<Laser>,
}

impl Spritable for Ship {
    fn update(&mut self) {
        self.sprite.update();

        self.heading += self.sprite.data.rotation;

        if self.is_boosting {
            let mut force = Vector::from_angle(self.heading);
            force.limit(0.15);
            self.sprite.data.velocity += force;
        }

        self.sprite.data.velocity.limit(10.0);
        self.sprite.data.velocity.mult(0.995);

        for laser in &mut self.lasers {
            laser.update();
        }

        self.lasers = self
            .lasers
            .drain(..)
            .filter(|laser| !laser.sprite.is_offscreen)
            .collect();

        if self.is_boosting && self.booster_flames_interval.is_ellapsed() {
            self.switch_flames = !self.switch_flames;
        }
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        for laser in &self.lasers {
            laser.draw(canvas.clone());
        }

        canvas.save();

        let position = self.sprite.data.position;
        let diameter = self.sprite.data.diameter;
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

        if self.is_boosting {
            self.draw_booster_flames(canvas.clone());
        }

        canvas.restore();
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        self.sprite.collide_with(sprite)
    }
}

impl Ship {
    pub fn new(sprite_data: SpriteData, is_plain_ship: bool, canvas: CanvasDimension) -> Ship {
        let mut booster_flames_interval = Interval::new();
        booster_flames_interval.set(BOOSTER_INTERVAL);

        let mut new_sprite_data = sprite_data;
        new_sprite_data.diameter = SHIP_SHELL_SIZE;

        Ship {
            sprite: Sprite::new(new_sprite_data, canvas),
            heading: -PI / 2.0,
            is_boosting: false,
            is_plain_ship,
            booster_flames_interval,
            switch_flames: false,
            lasers: Vec::new(),
        }
    }

    pub fn set_boost(&mut self, value: bool) {
        self.is_boosting = value;
    }

    pub fn set_rotation(&mut self, angle: f64) {
        self.sprite.data.rotation = angle;
    }

    pub fn shoot(&mut self) {
        let position = self.sprite.data.position;
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

    pub fn break_up(&mut self) -> Vec<Potatoid> {
        let mut new_potatoids = Vec::new();
        let potatoids_data = [
            [0.9, 0.05, 5.0],
            [0.9, -0.08, 6.0],
            [0.9, 0.15, 3.0],
            [0.7, -0.1, 5.0],
        ];

        for potatoid_data in potatoids_data {
            let mut sprite_data = self.sprite.data;

            sprite_data.diameter *= potatoid_data[0];
            sprite_data.velocity = Vector::random(-1.2, 1.2);
            sprite_data.rotation_step = potatoid_data[1];

            new_potatoids.push(Potatoid::new(
                sprite_data,
                potatoid_data[2] as u8,
                self.sprite.canvas,
            ));
        }

        self.lasers.clear();

        new_potatoids
    }

    fn draw_booster_flames(&self, canvas: CanvasRenderingContext2d) {
        let smaller_radius = (self.sprite.data.diameter / 10.0) * 3.5;
        let margin_x = self.sprite.data.diameter / 9.0;
        let margin_y = self.sprite.data.diameter / 3.0;
        let half_diameter = self.sprite.data.diameter / 2.0;

        if self.switch_flames {
            canvas.move_to(smaller_radius - margin_x, half_diameter);
            canvas.line_to(0.0, self.sprite.data.diameter);
            canvas.line_to(-smaller_radius + margin_x, half_diameter);
        } else {
            canvas.move_to(smaller_radius - margin_x, half_diameter);
            canvas.line_to(0.0, self.sprite.data.diameter - margin_y);
            canvas.line_to(-smaller_radius + margin_x, half_diameter);
        }

        canvas.stroke();
    }
}
