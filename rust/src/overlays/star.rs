use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, sprites::sprite::CanvasDimension, utils::map};

pub struct Star {
    x: f64,
    y: f64,
    z: f64,
    pz: f64,
    canvas: CanvasDimension,
}

impl Star {
    pub fn new(canvas: CanvasDimension) -> Star {
        let z = rand::thread_rng().gen_range(0.0..canvas.width);

        Star {
            x: rand::thread_rng().gen_range(-canvas.width..canvas.width),
            y: rand::thread_rng().gen_range(-canvas.height..canvas.height),
            z,
            pz: z,
            canvas,
        }
    }
}

impl Star {
    pub fn update(&mut self) {
        self.z -= 4.0;

        if self.z < 1.0 {
            self.x = rand::thread_rng().gen_range(-self.canvas.width..self.canvas.width);
            self.y = rand::thread_rng().gen_range(-self.canvas.height..self.canvas.height);
            self.z = self.canvas.width;
            self.pz = self.canvas.width;
        }
    }

    pub fn draw(&mut self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&Colors::Edge.value().into());

        let sx = map(self.x / self.z, 0.0, 1.0, 0.0, self.canvas.width);
        let sy = map(self.y / self.z, 0.0, 1.0, 0.0, self.canvas.height);
        let px = map(self.x / self.pz, 0.0, 1.0, 0.0, self.canvas.width);
        let py = map(self.y / self.pz, 0.0, 1.0, 0.0, self.canvas.height);

        self.pz = self.z;

        canvas.begin_path();
        canvas.move_to(px, py);
        canvas.line_to(sx, sy);
        canvas.close_path();

        canvas.stroke();

        canvas.restore();
    }
}
