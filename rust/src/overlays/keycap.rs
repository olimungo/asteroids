use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::utils::{colors::Colors, config::Config, vector::Vector};

pub struct Keycap {
    position: Vector,
    angle: f64,
    config: Config,
}

impl Keycap {
    pub fn new(position: Vector, angle: f64) -> Keycap {
        let config = Config::new();

        Keycap {
            position,
            angle,
            config,
        }
    }
}

impl Keycap {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let size = self.config.overlays.keycap.size;

        let half_size = size / 2.0;

        let _result = canvas.translate(self.position.x, self.position.y);
        let _result = canvas.rotate(PI / (180.0 / self.angle));

        canvas.stroke_rect(-half_size, -half_size, size, size);

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
