use web_sys::CanvasRenderingContext2d;

use crate::{
    game_states::GameState, sprites::sprite::CanvasDimension, utils::config::Config,
    utils::interval::Interval,
};

use super::{
    game_over::GameOver, game_paused::GamePaused, help::Help, homescreen::Homescreen, hub::Hub,
    level::Level, lifes::Lifes, new_life::NewLife, next_level::NextLevel, next_life::NextLife,
    score::Score, starfield::Starfield, top_score::TopScore,
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
    game_paused: GamePaused,
    next_level: NextLevel,
    next_life: NextLife,
    game_over: GameOver,
    score: Score,
    top_score: TopScore,
    level: Level,
    lifes: Lifes,
    new_life: NewLife,
    new_life_interval: Interval,
    scale_stage: bool,
    show_hub: bool,
    show_help: bool,
    show_starfield: bool,
    show_new_life: bool,
    canvas: CanvasDimension,
    config: Config,
}

impl OverlayManager {
    pub fn new(canvas: CanvasDimension) -> OverlayManager {
        let config = Config::new();

        OverlayManager {
            hub: Hub::new(canvas),
            homescreen: Homescreen::new(canvas),
            help: Help::new(canvas),
            starfield: Starfield::new(canvas),
            game_paused: GamePaused::new(canvas),
            next_level: NextLevel::new(canvas),
            next_life: NextLife::new(canvas),
            game_over: GameOver::new(canvas),
            score: Score::new(canvas),
            top_score: TopScore::new(canvas),
            level: Level::new(canvas),
            lifes: Lifes::new(canvas),
            new_life: NewLife::new(canvas),
            new_life_interval: Interval::new(),
            scale_stage: false,
            show_hub: false,
            show_help: false,
            show_starfield: false,
            show_new_life: false,
            canvas,
            config,
        }
    }
    pub fn update(&mut self, game_state: GameState) {
        match game_state {
            GameState::Homescreen => self.homescreen.update(),
            GameState::NextLife => self.next_life.update(),
            GameState::NextLevel => self.next_level.update(),
            GameState::GameOver => self.game_over.update(),
            _ => {}
        }

        if self.show_new_life {
            self.new_life.update();

            if self.new_life_interval.is_ellapsed() {
                self.show_new_life = false;
                self.new_life_interval.cancel();
            }
        }

        if self.show_starfield {
            self.starfield.update();
        }

        self.hub.update();
    }

    pub fn draw_foreground(&self, overlay_data: OverlayData) {
        let canvas = overlay_data.canvas;

        self.top_score.draw(overlay_data.top_score, canvas.clone());

        match overlay_data.game_state {
            GameState::Homescreen => self.homescreen.draw(canvas.clone()),
            GameState::NextLevel => self.next_level.draw(overlay_data.level, canvas.clone()),
            GameState::NextLife => self.next_life.draw(overlay_data.lifes, canvas.clone()),
            GameState::GameOver => self.game_over.draw(canvas.clone()),
            _ => {}
        }

        if overlay_data.game_state == GameState::Playing && overlay_data.is_game_paused {
            self.game_paused.draw(canvas.clone());
        }

        if overlay_data.game_state == GameState::Playing
            || overlay_data.game_state == GameState::NextLevel
        {
            self.lifes.draw(canvas.clone());
        }

        if overlay_data.game_state == GameState::Playing
            || overlay_data.game_state == GameState::GameOver
            || overlay_data.game_state == GameState::NextLevel
            || overlay_data.game_state == GameState::NextLife
        {
            self.score.draw(overlay_data.score, canvas.clone());
            self.level.draw(overlay_data.level, canvas.clone());
        }

        if self.show_new_life {
            self.new_life.draw(canvas.clone());
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

    pub fn dispaly_new_life(&mut self) {
        self.show_new_life = true;
        self.new_life_interval
            .set(self.config.overlays.manager.display_new_life_timeout);
    }

    pub fn set_life_count(&mut self, count: u32) {
        self.lifes.set_life_count(count);
    }

    fn set_scale(&self, ratio: f64, canvas: CanvasRenderingContext2d) {
        let _result = canvas.scale(1.0 / ratio, 1.0 / ratio);
        let _result = canvas.translate(
            (self.canvas.width * ratio - self.canvas.width) / 2.0,
            (self.canvas.height * ratio - self.canvas.height) / 2.0,
        );
    }
}
