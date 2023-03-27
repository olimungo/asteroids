extern crate web_sys;

use core::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::colors::Colors;
use crate::vector::Vector;

pub trait Spritable {
    fn update(&mut self);
    fn draw(&self, canvas: CanvasRenderingContext2d);
    fn collide_with(&self, sprite: Sprite) -> bool;
}

#[derive(Copy, Clone)]
pub struct SpriteData {
    pub position: Vector,
    pub velocity: Vector,
    pub diameter: f64,
    pub rotation: f64,
    pub rotation_step: f64,
}

#[derive(Copy, Clone)]
pub struct CanvasDimension {
    pub width: f64,
    pub height: f64,
}

#[derive(Copy)]
pub struct Sprite {
    pub sprite_data: SpriteData,
    pub canvas: CanvasDimension,
    pub is_offscreen: bool,
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Sprite {
            sprite_data: self.sprite_data,
            canvas: self.canvas,
            is_offscreen: self.is_offscreen,
        }
    }
}

impl Sprite {
    pub fn new(sprite_data: SpriteData, canvas: CanvasDimension) -> Sprite {
        Sprite {
            sprite_data,
            canvas,
            is_offscreen: false,
        }
    }

    fn check_window_edges(&mut self) -> bool {
        let radius = self.sprite_data.diameter / 2.0;
        let x = self.sprite_data.position.x;
        let y = self.sprite_data.position.y;
        let mut result = false;

        if x > self.canvas.width + radius {
            self.sprite_data.position.x = -radius;
            result = true;
        } else if x < -radius {
            self.sprite_data.position.x = self.canvas.width + radius;
            result = true;
        }

        if y > self.canvas.height + radius {
            self.sprite_data.position.y = -radius;
            result = true;
        } else if y < -radius {
            self.sprite_data.position.y = self.canvas.height + radius;
            result = true;
        }

        self.is_offscreen = result;

        result
    }
}

impl Spritable for Sprite {
    fn update(&mut self) {
        self.sprite_data.position += self.sprite_data.velocity;
        self.sprite_data.rotation += self.sprite_data.rotation_step;

        self.check_window_edges();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let _result = canvas.translate(self.sprite_data.position.x, self.sprite_data.position.y);
        let _result = canvas.rotate(self.sprite_data.rotation);

        canvas.begin_path();

        let _result = canvas.arc(0.0, 0.0, 40.0, 0.0, 2.0 * PI);

        canvas.fill();

        canvas.set_fill_style(&Colors::Dark.value().into());

        let _result = canvas.translate(-10.0, -10.0);

        canvas.fill_rect(0.0, 0.0, 20.0, 20.0);

        canvas.restore();
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        let distance = self
            .sprite_data
            .position
            .distance(sprite.sprite_data.position);

        if distance < self.sprite_data.diameter / 2.0 + sprite.sprite_data.diameter / 2.0 {
            return true;
        }

        false
    }
}
