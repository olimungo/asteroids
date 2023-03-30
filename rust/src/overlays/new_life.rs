use web_sys::CanvasRenderingContext2d;

use crate::{sprites::sprite::CanvasDimension, utils::millis};

pub struct NewLife {
    past_millis: f64,
    display_new_life: bool,
    center_x: f64,
}

impl NewLife {
    pub fn new(canvas: CanvasDimension) -> NewLife {
        NewLife {
            past_millis: 0.0,
            display_new_life: false,
            center_x: canvas.width / 2.0,
        }
    }
}

impl NewLife {
    pub fn update(&mut self) {
        let now = millis();

        if now - self.past_millis > 1000.0 {
            self.display_new_life = !self.display_new_life;
            self.past_millis = now;
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        if self.display_new_life {
            canvas.save();

            canvas.set_text_align("center");
            canvas.set_font("300 20px 'Exo 2'");
            let _result = canvas.fill_text("NEW LIFE!", self.center_x, 85.0);

            canvas.restore();
        }
    }
}
