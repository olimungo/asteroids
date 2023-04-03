use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{
    sprites::sprite::CanvasDimension,
    utils::{colors::Colors, javascript::fps},
};

pub struct Hub {
    center_x: f64,
    center_y: f64,
    frame_count: u32,
    fps: String,
    canvas: CanvasDimension,
}

impl Hub {
    pub fn new(canvas: CanvasDimension) -> Hub {
        Hub {
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
            frame_count: 0,
            fps: "0".to_string(),
            canvas,
        }
    }
}

impl Hub {
    pub fn update(&mut self) {
        let fps = fps();

        self.frame_count += 1;

        if self.frame_count > 50 {
            self.frame_count = 0;
            self.fps = format!("{}", fps);
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&Colors::Dark.value().into());

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

        canvas.set_text_align("left");
        canvas.set_font("300 30px 'Exo 2'");
        canvas.set_fill_style(&Colors::Dark.value().into());
        let _result = canvas.fill_text(&self.fps, 30.0, self.canvas.height - 45.0);
        let _result = canvas.fill_text("FPS", 75.0, self.canvas.height - 45.0);

        canvas.restore();
    }
}
