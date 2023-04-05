use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::CanvasDimension;

pub struct Level {
    canvas: CanvasDimension,
}

impl Level {
    pub fn new(canvas: CanvasDimension) -> Level {
        Level { canvas }
    }
}

impl Level {
    pub fn draw(&self, level: u32, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("center");
        canvas.set_font("300 25px 'Exo 2'");

        let text = format!("LEVEL {}", level);
        let _result = canvas.fill_text(&text, self.canvas.width / 2.0, 55.0);

        canvas.restore();
    }
}
