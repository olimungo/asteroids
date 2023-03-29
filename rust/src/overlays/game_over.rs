use web_sys::CanvasRenderingContext2d;

use crate::{sprites::sprite::CanvasDimension, utils::millis};

pub struct GameOver {
    past_millis: f64,
    display_press_s: bool,
    center_x: f64,
    center_y: f64,
}

impl GameOver {
    pub fn new(canvas: CanvasDimension) -> GameOver {
        GameOver {
            past_millis: 0.0,
            display_press_s: false,
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl GameOver {
    pub fn update(&mut self) {
        let now = millis();

        if now - self.past_millis > 1000.0 {
            self.display_press_s = !self.display_press_s;
            self.past_millis = now;
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("center");
        canvas.set_font("100 80px 'Exo 2'");

        let _result = canvas.fill_text("GAME OVER", self.center_x, self.center_y);

        if self.display_press_s {
            canvas.set_font("100 25px 'Exo 2'");

            let _result = canvas.fill_text(
                "PRESS \"S\" TO PLAY AGAIN",
                self.center_x,
                self.center_y + 50.0,
            );
        }

        canvas.restore();
    }
}
