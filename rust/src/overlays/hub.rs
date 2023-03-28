use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, sprites::sprite::CanvasDimension};

pub struct Hub {
    canvas: CanvasDimension,
    center_x: f64,
    center_y: f64,
}

impl Hub {
    pub fn new(canvas: CanvasDimension) -> Hub {
        Hub {
            canvas,
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl Hub {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&Colors::Warning.value().into());

        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.0, 0.0, self.canvas.width, self.canvas.height);

        canvas.begin_path();

        canvas.move_to(self.center_x, 0.0);
        canvas.line_to(self.center_x, self.canvas.height);
        canvas.move_to(0.0, self.center_y);
        canvas.line_to(self.canvas.width, self.center_y);

        canvas.close_path();

        canvas.stroke();

        canvas.begin_path();

        let _result = canvas.arc(self.center_x, self.center_y, 200.0, 0.0, 2.0 * PI);

        canvas.close_path();

        canvas.stroke();

        canvas.restore();
    }
}
