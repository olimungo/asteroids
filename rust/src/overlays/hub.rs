use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

pub struct Hub {
    width: f64,
    height: f64,
    center_x: f64,
    center_y: f64,
}

impl Hub {
    pub fn new(width: f64, height: f64) -> Hub {
        Hub {
            width,
            height,
            center_x: width / 2.0,
            center_y: height / 2.0,
        }
    }
}

impl Hub {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&"#00ff00".into());
        canvas.set_line_width(1.0);
        canvas.stroke_rect(20.0, 20.0, self.width - 40.0, self.height - 40.0);

        canvas.begin_path();

        canvas.move_to(self.center_x, 20.0);
        canvas.line_to(self.center_x, self.height - 20.0);
        canvas.move_to(20.0, self.center_y);
        canvas.line_to(self.width - 20.0, self.center_y);

        canvas.close_path();

        canvas.stroke();

        canvas.begin_path();

        let _result = canvas.arc(self.center_x, self.center_y, 200.0, 0.0, 2.0 * PI);

        canvas.close_path();

        canvas.stroke();

        canvas.restore();
    }
}
