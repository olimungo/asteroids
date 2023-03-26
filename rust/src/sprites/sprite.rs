extern crate web_sys;

use core::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;

use crate::vector::Vector;

static DEAD_COLOR: &str = "#242c37";
static ALIVE_COLOR: &str = "#4fe4c1";

pub trait Spritable {
    fn update(&mut self);
    fn draw(&self);
    fn check_window_edges(&self) -> bool;
    fn collide_width(&self) -> bool;
}

pub struct Sprite<T: Spritable> {
    pub position: Vector,
    pub velocity: Vector,
    pub diameter: f64,
    pub rotation: f64,
    pub rotation_step: f64,
    pub sprite: Option<Box<T>>,
    canvas: Option<CanvasRenderingContext2d>,
}

impl<T: Spritable> Sprite<T> {
    pub fn new(sprite: T, canvas: Option<CanvasRenderingContext2d>) -> Sprite<T> {
        Sprite {
            position: Vector::new(50.0, 50.0),
            velocity: Vector::new(0.1, 0.1),
            diameter: 50.0,
            rotation: 0.0,
            rotation_step: 0.02,
            sprite: Some(Box::new(sprite)),
            canvas,
        }
    }
}

impl<T: Spritable> Spritable for Sprite<T> {
    fn update(&mut self) {
        self.position = self.position + self.velocity;
        self.rotation += self.rotation_step;
    }

    fn draw(&self) {
        if let Some(canvas) = &self.canvas {
            canvas.save();

            let _result = canvas.translate(self.position.x, self.position.y);
            let _result = canvas.rotate(self.rotation);

            canvas.set_fill_style(&ALIVE_COLOR.into());
            let _result = canvas.arc(0f64, 0f64, self.diameter / 2.0, 0f64, 2f64 * PI);
            canvas.fill();

            canvas.set_fill_style(&DEAD_COLOR.into());
            let _result = canvas.translate(-10.0, -10.0);

            canvas.fill_rect(0f64, 0f64, 20f64, 20f64);

            canvas.restore();
        }
    }

    fn check_window_edges(&self) -> bool {
        false
    }

    fn collide_width(&self) -> bool {
        false
    }
}

// this.p5.translate(this.position.x, this.position.y);
// this.p5.rotate(this.rotation);

// this.p5.fill(Colors.EDGE);
// this.p5.noStroke();

// this.p5.ellipse(0, 0, this.diameter);

// this.p5.fill(Colors.DARK);
// this.p5.rectMode(this.p5.CENTER);
// this.p5.rect(0, 0, this.diameter / 4, this.diameter / 4);
