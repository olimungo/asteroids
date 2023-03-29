use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, sprites::sprite::CanvasDimension, vector::Vector};

use super::{keycap::Keycap, spacebar::Spacebar};

pub struct Help {
    keycap_left: Keycap,
    keycap_right: Keycap,
    keycap_up: Keycap,
    spacebar: Spacebar,
    center_x: f64,
    canvas: CanvasDimension,
}

impl Help {
    pub fn new(canvas: CanvasDimension) -> Help {
        Help {
            keycap_left: Keycap::new(Vector::new(650.0, 280.0), -90.0),
            keycap_right: Keycap::new(Vector::new(770.0, 285.0), 90.0),
            keycap_up: Keycap::new(Vector::new(710.0, 215.0), 0.0),
            spacebar: Spacebar::new(Vector::new(710.0, 365.0)),
            center_x: canvas.width / 2.0,
            canvas,
        }
    }
}

impl Help {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_fill_style(&Colors::Background.value().into());

        canvas.fill_rect(
            150.0,
            150.0,
            self.canvas.width - 300.0,
            self.canvas.height - 300.0,
        );

        canvas.stroke_rect(
            150.0,
            150.0,
            self.canvas.width - 300.0,
            self.canvas.height - 300.0,
        );

        self.keycap_left.draw(canvas.clone());
        self.keycap_right.draw(canvas.clone());
        self.keycap_up.draw(canvas.clone());
        self.spacebar.draw(canvas.clone());

        canvas.set_fill_style(&Colors::Edge.value().into());
        canvas.set_font("300 15px 'Exo 2'");

        canvas.set_text_align("left");
        let _result = canvas.fill_text("INSERT COIN", 220.0, 275.0);
        let _result = canvas.fill_text("PAUSE", 220.0, 325.0);
        let _result = canvas.fill_text("HUB", 220.0, 375.0);
        let _result = canvas.fill_text("SHOW EGDES", 220.0, 425.0);
        let _result = canvas.fill_text("STARFIELD", 220.0, 475.0);

        let _result = canvas.fill_text("ASTEROID", 600.0, 450.0);
        let _result = canvas.fill_text("UFO", 600.0, 475.0);
        let _result = canvas.fill_text("NEW LIFE", 600.0, 500.0);

        let _result = canvas.fill_text("10 POINTS", 745.0, 450.0);
        let _result = canvas.fill_text("50 POINTS", 745.0, 475.0);
        let _result = canvas.fill_text("3000 POINTS", 745.0, 500.0);

        canvas.set_text_align("right");
        let _result = canvas.fill_text("S", 475.0, 275.0);
        let _result = canvas.fill_text("P", 475.0, 325.0);
        let _result = canvas.fill_text("U", 475.0, 375.0);
        let _result = canvas.fill_text("E", 475.0, 425.0);
        let _result = canvas.fill_text("X", 475.0, 475.0);

        canvas.set_text_align("center");
        let _result = canvas.fill_text("BOOST", 710.0, 250.0);
        let _result = canvas.fill_text("TURN LEFT", 650.0, 320.0);
        let _result = canvas.fill_text("TURN RIGHT", 770.0, 320.0);
        let _result = canvas.fill_text("SHOOT", 710.0, 400.0);

        canvas.set_font("300 35px 'Exo 2'");
        let _result = canvas.fill_text("HELP", self.center_x, 210.0);

        canvas.restore();
    }
}
