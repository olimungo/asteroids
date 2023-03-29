use web_sys::CanvasRenderingContext2d;

use crate::{game_states::GameState, sprites::sprite::CanvasDimension};

use super::{
    game_over::GameOver, help::Help, homescreen::Homescreen, hub::Hub, level::Level,
    next_level::NextLevel, next_life::NextLife, pause::Pause, score::Score, starfield::Starfield,
};

pub struct OverlayData {
    pub game_state: GameState,
    pub is_game_paused: bool,
    pub top_score: u32,
    pub score: u32,
    pub level: u32,
    pub lifes: u32,
    pub canvas: CanvasRenderingContext2d,
}

pub struct OverlayManager {
    hub: Hub,
    homescreen: Homescreen,
    help: Help,
    starfield: Starfield,
    pause: Pause,
    next_level: NextLevel,
    next_life: NextLife,
    game_over: GameOver,
    score: Score,
    level: Level,
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
            help: Help::new(canvas),
            starfield: Starfield::new(canvas),
            pause: Pause::new(canvas),
            next_level: NextLevel::new(canvas),
            next_life: NextLife::new(canvas),
            game_over: GameOver::new(canvas),
            score: Score::new(canvas),
            level: Level::new(canvas),
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
            GameState::NextLife => self.next_life.update(),
            GameState::NextLevel => self.next_level.update(),
            GameState::GameOver => self.game_over.update(),
        }

        if self.show_starfield {
            self.starfield.update();
        }
    }

    pub fn draw_foreground(&self, overlay_data: OverlayData) {
        let canvas = overlay_data.canvas;

        match overlay_data.game_state {
            GameState::Homescreen => self.homescreen.draw(canvas.clone()),
            GameState::NextLevel => self.next_level.draw(overlay_data.level, canvas.clone()),
            GameState::NextLife => self.next_life.draw(overlay_data.lifes, canvas.clone()),
            GameState::GameOver => self.game_over.draw(canvas.clone()),
            _ => {}
        }

        if overlay_data.game_state == GameState::Playing {
            if overlay_data.is_game_paused {
                self.pause.draw(canvas.clone());
            }

            self.score.draw(overlay_data.score, canvas.clone());
            self.level.draw(overlay_data.level, canvas.clone());

            // todo!("overlays: lifes");
        }

        if self.show_new_life {
            self.next_life.draw(overlay_data.lifes, canvas.clone());
        }

        if self.show_help {
            self.help.draw(canvas.clone());
        }

        if self.show_hub {
            self.hub.draw(canvas);
        }
    }

    pub fn draw_background(&mut self, canvas: CanvasRenderingContext2d) {
        if self.scale_stage {
            self.set_scale(1.3, canvas.clone());
        } else {
            self.set_scale(1.0, canvas.clone());
        }

        if self.show_starfield {
            self.starfield.draw(canvas);
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        match key {
            "e" => self.scale_stage = !self.scale_stage,
            "h" => self.show_help = !self.show_help,
            "u" => self.show_hub = !self.show_hub,
            "x" => self.show_starfield = !self.show_starfield,
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
