use web_sys::CanvasRenderingContext2d;

use crate::{
    game_states::GameState,
    overlays::overlays_manager::{OverlayData, OverlayManager},
    sprites::{sprite::CanvasDimension, sprites_manager::SpriteManager},
    utils::{
        colors::Colors,
        config::Config,
        interval::Interval,
        javascript::{getCookie, setCookie},
    },
};

const GAME_OVER_STATE_TIMEOUT: u32 = 8000; // ms
const ADD_LIFE_WHEN_SCORED: u32 = 3000;
const ASTEROID_HIT_SCORE: u32 = 10;
const UFO_HIT_SCORE: u32 = 50;

pub struct GameManager {
    game_state: GameState,
    sprite_manager: SpriteManager,
    overlay_manager: OverlayManager,
    is_game_paused: bool,
    level: u32,
    lifes: u32,
    top_score: u32,
    score: u32,
    max_asteroids: u32,
    ufo_create_frequency: u32,
    ufo_shoot_frequency: u32,
    life_added_so_far: u32,
    game_over_interval: Interval,
    canvas: CanvasDimension,
    config: Config,
}

impl GameManager {
    pub fn new(canvas: CanvasDimension) -> GameManager {
        let top_score = getCookie();

        let mut sprite_manager = SpriteManager::new(canvas);
        sprite_manager.create_ufo(0);

        GameManager {
            game_state: GameState::Homescreen,
            sprite_manager,
            overlay_manager: OverlayManager::new(canvas),
            is_game_paused: false,
            level: 0,
            lifes: 0,
            top_score,
            score: 0,
            max_asteroids: 0,
            ufo_create_frequency: 0,
            ufo_shoot_frequency: 0,
            life_added_so_far: 0,
            game_over_interval: Interval::new(),
            canvas,
            config: Config::new(),
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        self.overlay_manager.key_pressed(key);

        if (self.game_state == GameState::Homescreen || self.game_state == GameState::GameOver)
            && key == "s"
        {
            self.start_game();
        }

        if self.game_state == GameState::Playing {
            self.sprite_manager.key_pressed(key);

            if key == "p" {
                self.is_game_paused = !self.is_game_paused;

                if self.is_game_paused {
                    self.sprite_manager.pause();
                } else {
                    self.sprite_manager.unpause();
                }
            }
        }

        if self.game_state == GameState::NextLevel && key == "s" {
            self.next_level();
        }

        if self.game_state == GameState::NextLife && key == "s" {
            self.start_level();
        }
    }

    pub fn key_released(&mut self, key: &str) {
        self.sprite_manager.key_released(key);
    }

    pub fn update(&mut self) {
        self.overlay_manager.update(self.game_state);

        if !self.is_game_paused {
            self.sprite_manager.update();

            if self.game_state == GameState::Playing {
                self.check_level();
                self.check_new_life();
            }
        }

        if self.game_over_interval.is_ellapsed() {
            self.game_over_interval.cancel();
            self.game_state = GameState::Homescreen;

            self.sprite_manager.reset();
            self.sprite_manager.create_ufo(0);
            self.sprite_manager
                .create_asteroids(self.config.game.asteroids_start_count);
        }
    }

    pub fn draw(&mut self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        // Clear canvas
        canvas.set_stroke_style(&Colors::Edge.value().into());
        canvas.set_fill_style(&Colors::Background.value().into());
        canvas.set_line_width(1.5);

        canvas.fill_rect(0f64, 0f64, self.canvas.width, self.canvas.height);

        canvas.set_fill_style(&Colors::Edge.value().into());

        self.overlay_manager.draw_background(canvas.clone());

        self.sprite_manager.draw(canvas.clone());

        let overlay_data = OverlayData {
            game_state: self.game_state,
            is_game_paused: self.is_game_paused,
            top_score: self.top_score,
            score: self.get_score(),
            level: self.level,
            lifes: self.lifes,
            canvas: canvas.clone(),
        };

        self.overlay_manager.draw_foreground(overlay_data);

        canvas.restore();
    }

    fn start_game(&mut self) {
        self.game_over_interval.cancel();

        self.score = 0;
        self.level = 1;

        self.score = 0;
        self.level = 1;
        self.max_asteroids = self.config.game.asteroids_start_count;
        self.lifes = self.config.game.lifes_when_starting;

        self.overlay_manager.set_life_count(self.lifes - 1);

        self.life_added_so_far = 0;
        self.ufo_create_frequency = self.config.game.ufo_create_init_frequency;
        self.ufo_shoot_frequency = self.config.game.ufo_shoot_init_frequency;

        self.sprite_manager.reset();

        self.start_level();
    }

    fn start_level(&mut self) {
        self.game_state = GameState::Playing;

        self.sprite_manager.start_level(
            self.max_asteroids,
            self.ufo_create_frequency,
            self.ufo_shoot_frequency,
        );
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.max_asteroids += self.config.game.asteroids_level_increment;
        self.ufo_create_frequency -= self.config.game.ufo_create_decrement_frequency;
        self.ufo_shoot_frequency -= self.config.game.ufo_shoot_decrement_frequency;

        if self.ufo_create_frequency < self.config.game.ufo_create_minimal_frequency {
            self.ufo_create_frequency = self.config.game.ufo_create_minimal_frequency;
        }

        if self.ufo_shoot_frequency < self.config.game.ufo_shoot_minimal_frequency {
            self.ufo_shoot_frequency = self.config.game.ufo_shoot_minimal_frequency;
        }

        self.start_level();
    }

    fn check_level(&mut self) {
        if self.sprite_manager.is_ship_active {
            if self.sprite_manager.get_asteroids_count() == 0 {
                self.game_state = GameState::NextLevel;
                self.score = self.get_score();
                self.sprite_manager.stop_level();
            }
        } else {
            self.lifes -= 1;
            self.score = self.get_score();
            self.sprite_manager.stop_level();

            if self.lifes == 0 {
                self.game_state = GameState::GameOver;
                self.game_over_interval.set(GAME_OVER_STATE_TIMEOUT);

                if self.score > self.top_score {
                    self.top_score = self.score;
                    setCookie(self.top_score);
                }
            } else {
                self.game_state = GameState::NextLife;
                self.overlay_manager.set_life_count(self.lifes - 1);
            }
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
            + self.sprite_manager.count_asteroids_hit * ASTEROID_HIT_SCORE
            + self.sprite_manager.count_ufo_hit * UFO_HIT_SCORE
    }

    pub fn check_new_life(&mut self) {
        if self.get_score() - self.life_added_so_far * ADD_LIFE_WHEN_SCORED >= ADD_LIFE_WHEN_SCORED
        {
            self.life_added_so_far += 1;
            self.lifes += 1;
            self.overlay_manager.set_life_count(self.lifes - 1);
            self.overlay_manager.dispaly_new_life();
        }
    }
}
