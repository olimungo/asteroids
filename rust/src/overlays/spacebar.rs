use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, vector::Vector};

const WIDTH: f64 = 200.0;
const HEIGHT: f64 = 30.0;

pub struct Spacebar {
    position: Vector,
}

impl Spacebar {
    pub fn new(position: Vector) -> Spacebar {
        Spacebar { position }
    }
}

impl Spacebar {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let _result = canvas.translate(
            self.position.x - WIDTH / 2.0,
            self.position.y - HEIGHT / 2.0,
        );

        canvas.set_fill_style(&Colors::Background.value().into());
        canvas.fill_rect(0.0, 0.0, WIDTH, HEIGHT);

        canvas.stroke_rect(0.0, 0.0, WIDTH, HEIGHT);

        canvas.set_fill_style(&Colors::Edge.value().into());
        canvas.set_text_align("center");
        canvas.set_font("300 15px 'Exo 2'");
        let _result = canvas.fill_text("SPACEBAR", WIDTH / 2.0, HEIGHT / 2.0 + 5.0);

        canvas.restore();
    }
}
