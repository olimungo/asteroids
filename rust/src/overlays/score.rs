use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::CanvasDimension;

pub struct Score {
    canvas: CanvasDimension,
}

impl Score {
    pub fn new(canvas: CanvasDimension) -> Score {
        Score { canvas }
    }
}

impl Score {
    pub fn draw(&self, score: u32, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("right");
        canvas.set_font("300 25px 'Exo 2'");

        let text = format!("{}", score);
        let _result = canvas.fill_text(&text, self.canvas.width - 50.0, 55.0);

        canvas.restore();
    }
}
