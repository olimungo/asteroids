use web_sys::CanvasRenderingContext2d;

use crate::{sprites::sprite::CanvasDimension, utils::javascript::millis};

pub struct NextLevel {
    past_millis: f64,
    display_press_s: bool,
    center_x: f64,
    center_y: f64,
}

impl NextLevel {
    pub fn new(canvas: CanvasDimension) -> NextLevel {
        NextLevel {
            past_millis: 0.0,
            display_press_s: false,
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl NextLevel {
    pub fn update(&mut self) {
        let now = millis();

        if now - self.past_millis > 1000.0 {
            self.display_press_s = !self.display_press_s;
            self.past_millis = now;
        }
    }

    pub fn draw(&self, level: u32, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("center");
        canvas.set_font("100 60px 'Exo 2'");
        let text = format!("LEVEL {} COMPLETED", level);
        let _result = canvas.fill_text(&text, self.center_x, self.center_y);

        if self.display_press_s {
            canvas.set_font("100 25px 'Exo 2'");

            let _result = canvas.fill_text(
                "PRESS \"S\" TO GO TO THE NEXT LEVEL",
                self.center_x,
                self.center_y + 50.0,
            );
        }

        canvas.restore();
    }
}
