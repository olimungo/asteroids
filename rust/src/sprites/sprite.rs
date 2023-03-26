extern crate web_sys;

use core::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::vector::Vector;

static DEAD_COLOR: &str = "#242c37";
static ALIVE_COLOR: &str = "#4fe4c1";

pub trait Spritable {
    fn update(&mut self);
    fn draw(&self, canvas: CanvasRenderingContext2d);
    fn check_window_edges(&mut self) -> bool;
    fn collide_width(&self) -> bool;
}

pub struct SpriteData {
    pub position: Vector,
    pub velocity: Vector,
    pub diameter: f64,
    pub rotation: f64,
    pub rotation_step: f64,
}

pub struct Canvas {
    pub width: f64,
    pub height: f64,
}

pub struct Sprite {
    pub sprite_data: SpriteData,
    pub canvas: Canvas,
}

impl Sprite {
    pub fn new(sprite_data: SpriteData, canvas: Canvas) -> Sprite {
        Sprite {
            sprite_data,
            canvas,
        }
    }
}

impl Spritable for Sprite {
    fn update(&mut self) {
        self.sprite_data.position = self.sprite_data.position + self.sprite_data.velocity;
        self.sprite_data.rotation += self.sprite_data.rotation_step;

        self.check_window_edges();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.begin_path();

        let _result = canvas.translate(self.sprite_data.position.x, self.sprite_data.position.y);
        let _result = canvas.rotate(self.sprite_data.rotation);

        canvas.set_fill_style(&ALIVE_COLOR.into());
        let _result = canvas.arc(0f64, 0f64, self.sprite_data.diameter / 2.0, 0f64, 2f64 * PI);
        canvas.fill();

        canvas.set_fill_style(&DEAD_COLOR.into());
        let _result = canvas.translate(-10.0, -10.0);

        canvas.fill_rect(0f64, 0f64, 20f64, 20f64);

        canvas.close_path();

        canvas.restore();
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

        result
    }

    fn collide_width(&self) -> bool {
        false
    }
}
