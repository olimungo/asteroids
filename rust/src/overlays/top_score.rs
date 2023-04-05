use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::CanvasDimension;

pub struct TopScore {
    canvas: CanvasDimension,
}

impl TopScore {
    pub fn new(canvas: CanvasDimension) -> TopScore {
        TopScore { canvas }
    }
}

impl TopScore {
    pub fn draw(&self, score: u32, canvas: CanvasRenderingContext2d) {
        if score > 0 {
            canvas.save();

            canvas.set_text_align("center");
            canvas.set_font("300 25px 'Exo 2'");

            let text = format!("TOP   {}", score);
            let _result = canvas.fill_text(&text, self.canvas.width / 2.0 + 220.0, 55.0);

            canvas.restore();
        }
    }
}
