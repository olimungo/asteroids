use web_sys::CanvasRenderingContext2d;

use crate::utils::{colors::Colors, config::Config, vector::Vector};

pub struct Spacebar {
    position: Vector,
    config: Config,
}

impl Spacebar {
    pub fn new(position: Vector) -> Spacebar {
        let config = Config::new();

        Spacebar { position, config }
    }
}

impl Spacebar {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let width = self.config.overlays.spacebar.width;
        let height = self.config.overlays.spacebar.height;

        let _result = canvas.translate(
            self.position.x - width / 2.0,
            self.position.y - height / 2.0,
        );

        canvas.set_fill_style(&Colors::Background.value().into());
        canvas.fill_rect(0.0, 0.0, width, height);

        canvas.stroke_rect(0.0, 0.0, width, height);

        canvas.set_fill_style(&Colors::Edge.value().into());
        canvas.set_text_align("center");
        canvas.set_font("300 15px 'Exo 2'");
        let _result = canvas.fill_text("SPACEBAR", width / 2.0, height / 2.0 + 5.0);

        canvas.restore();
    }
}
