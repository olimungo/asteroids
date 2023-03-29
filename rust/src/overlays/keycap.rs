use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, vector::Vector};

const SIZE: f64 = 30.0;

pub struct Keycap {
    position: Vector,
    angle: f64,
}

impl Keycap {
    pub fn new(position: Vector, angle: f64) -> Keycap {
        Keycap { position, angle }
    }
}

impl Keycap {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let half_size = SIZE / 2.0;

        let _result = canvas.translate(self.position.x, self.position.y);
        let _result = canvas.rotate(PI / (180.0 / self.angle));

        canvas.stroke_rect(-half_size, -half_size, SIZE, SIZE);

        canvas.set_fill_style(&Colors::Edge.value().into());

        canvas.begin_path();
        canvas.move_to(0.0, -half_size + 8.0);
        canvas.line_to(half_size - 8.0, half_size - 8.0);
        canvas.line_to(-half_size + 8.0, half_size - 8.0);
        canvas.close_path();

        canvas.fill();

        canvas.restore();
    }
}
