extern crate web_sys;

use core::f64::consts::PI;
use rand::Rng;
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

pub struct Sprite<T: Spritable> {
    pub position: Vector,
    pub velocity: Vector,
    pub diameter: f64,
    pub rotation: f64,
    pub rotation_step: f64,
    pub sprite: Option<Box<T>>,
    canvas_width: f64,
    canvas_height: f64,
}

impl<T: Spritable> Sprite<T> {
    pub fn new(sprite: T, canvas_width: f64, canvas_height: f64) -> Sprite<T> {
        let mut position = Vector::random();
        position = position + Vector::new(canvas_width / 2.0, canvas_height / 2.0);

        Sprite {
            position,
            velocity: Vector::random(),
            diameter: 50.0,
            rotation: 0.0,
            rotation_step: rand::thread_rng().gen_range(-0.05..0.05),
            sprite: Some(Box::new(sprite)),
            canvas_width,
            canvas_height,
        }
    }
}

impl<T: Spritable> Spritable for Sprite<T> {
    fn update(&mut self) {
        self.position = self.position + self.velocity;
        self.rotation += self.rotation_step;

        self.check_window_edges();

        // let x = &self.sprite.unwrap();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.begin_path();

        let _result = canvas.translate(self.position.x, self.position.y);
        let _result = canvas.rotate(self.rotation);

        canvas.set_fill_style(&ALIVE_COLOR.into());
        let _result = canvas.arc(0f64, 0f64, self.diameter / 2.0, 0f64, 2f64 * PI);
        canvas.fill();

        canvas.set_fill_style(&DEAD_COLOR.into());
        let _result = canvas.translate(-10.0, -10.0);

        canvas.fill_rect(0f64, 0f64, 20f64, 20f64);

        canvas.close_path();

        canvas.restore();
    }

    fn check_window_edges(&mut self) -> bool {
        let radius = self.diameter / 2.0;
        let x = self.position.x;
        let y = self.position.y;
        let mut result = false;

        if x > self.canvas_width + radius {
            self.position.x = -radius;
            result = true;
        } else if x < -radius {
            self.position.x = self.canvas_width + radius;
            result = true;
        }

        if y > self.canvas_height + radius {
            self.position.y = -radius;
            result = true;
        } else if y < -radius {
            self.position.y = self.canvas_height + radius;
            result = true;
        }

        result
    }

    fn collide_width(&self) -> bool {
        false
    }
}
