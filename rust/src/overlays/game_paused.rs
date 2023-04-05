use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::CanvasDimension;

pub struct GamePaused {
    center_x: f64,
    center_y: f64,
}

impl GamePaused {
    pub fn new(canvas: CanvasDimension) -> GamePaused {
        GamePaused {
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl GamePaused {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("center");
        canvas.set_font("100 60px 'Exo 2'");
        let _result = canvas.fill_text("GAME PAUSED", self.center_x, self.center_y - 100.0);

        canvas.restore();
    }
}
