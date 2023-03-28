use web_sys::CanvasRenderingContext2d;

use crate::{game_states::GameState, sprites::sprite::CanvasDimension};

use super::{homescreen::Homescreen, hub::Hub};

pub struct OverlayManager {
    hub: Hub,
    homescreen: Homescreen,
    scale_stage: bool,
    show_hub: bool,
    show_help: bool,
    show_starfield: bool,
    show_new_life: bool,
    canvas: CanvasDimension,
}

impl OverlayManager {
    pub fn new(canvas: CanvasDimension) -> OverlayManager {
        OverlayManager {
            hub: Hub::new(canvas),
            homescreen: Homescreen::new(canvas),
            scale_stage: false,
            show_hub: false,
            show_help: false,
            show_starfield: false,
            show_new_life: false,
            canvas,
        }
    }
    pub fn update(&mut self, game_state: GameState) {
        match game_state {
            GameState::Homescreen => self.homescreen.update(),
            GameState::Playing => {}
            GameState::NextLife => {}
            GameState::NextLevel => {}
            GameState::GameOver => {}
        }
    }

    pub fn draw_foreground(&self, game_state: GameState, canvas: CanvasRenderingContext2d) {
        match game_state {
            GameState::Homescreen => self.homescreen.draw(canvas.clone()),
            GameState::Playing => {}
            GameState::NextLife => {}
            GameState::NextLevel => {}
            GameState::GameOver => {}
        }

        if self.show_new_life {
            // self.new_life.draw(canvas);
        }

        if self.show_help {
            // self.help.draw(canvas);
        }

        if self.show_hub {
            self.hub.draw(canvas);
        }
    }

    pub fn draw_background(&self, canvas: CanvasRenderingContext2d) {
        if self.scale_stage {
            self.set_scale(1.3, canvas);
        } else {
            self.set_scale(1.0, canvas);
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        match key {
            "e" => self.scale_stage = !self.scale_stage,
            "h" => self.show_help = !self.show_help,
            "u" => self.show_hub = !self.show_hub,
            "x" => self.show_starfield = !self.scale_stage,
            _ => {}
        }
    }

    fn set_scale(&self, ratio: f64, canvas: CanvasRenderingContext2d) {
        let _result = canvas.scale(1.0 / ratio, 1.0 / ratio);
        let _result = canvas.translate(
            (self.canvas.width * ratio - self.canvas.width) / 2.0,
            (self.canvas.height * ratio - self.canvas.height) / 2.0,
        );
    }
}
