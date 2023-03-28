use web_sys::CanvasRenderingContext2d;

use crate::{sprites::sprite::CanvasDimension, utils::millis};

pub struct Homescreen {
    past_millis: f64,
    display_insert_coin: bool,
    center_x: f64,
    center_y: f64,
}

impl Homescreen {
    pub fn new(canvas: CanvasDimension) -> Homescreen {
        Homescreen {
            past_millis: 0.0,
            display_insert_coin: false,
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl Homescreen {
    pub fn update(&mut self) {
        let now = millis();

        if now - self.past_millis > 1000.0 {
            self.display_insert_coin = !self.display_insert_coin;
            self.past_millis = now;
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_text_align("center");
        canvas.set_font("100 100px 'Exo 2'");
        let _result = canvas.fill_text("ASTEROIDS", self.center_x, self.center_y - 100.0);

        canvas.set_font("300 15px 'Exo 2'");
        let _result = canvas.fill_text("by olimungo", self.center_x, self.center_y - 65.0);

        if self.display_insert_coin {
            canvas.set_font("100 40px 'Exo 2'");
            let _result = canvas.fill_text("INSERT 1 COIN", self.center_x, self.center_y + 50.0);
        }

        canvas.set_font("10 25px 'Exo 2'");
        let _result =
            canvas.fill_text("PRESS \"H\" FOR HELP", self.center_x, self.center_y + 250.0);

        canvas.restore();
    }
}
