use crate::sprites::sprite::Spritable;
use web_sys::CanvasRenderingContext2d;

static DEAD_COLOR: &str = "#ff0000";
static ALIVE_COLOR: &str = "#00ff00";

pub struct Potatoid {}

impl Spritable for Potatoid {
    fn update(&mut self) {}

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        // canvas.save();

        // canvas.begin_path();

        // let _result = canvas.translate(self.position.x, self.position.y);
        // let _result = canvas.rotate(self.rotation);

        // canvas.set_fill_style(&ALIVE_COLOR.into());
        // let _result = canvas.arc(0f64, 0f64, self.diameter / 2.0, 0f64, 2f64 * PI);
        // canvas.fill();

        // canvas.set_fill_style(&DEAD_COLOR.into());
        // let _result = canvas.translate(-10.0, -10.0);

        // canvas.fill_rect(0f64, 0f64, 20f64, 20f64);

        // canvas.close_path();

        // canvas.restore();
    }

    fn check_window_edges(&mut self) -> bool {
        false
    }

    fn collide_width(&self) -> bool {
        false
    }
}

impl Potatoid {
    pub fn new() -> Potatoid {
        Potatoid {}
    }
}
