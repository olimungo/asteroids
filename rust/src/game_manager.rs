use web_sys::CanvasRenderingContext2d;

use crate::{
    colors::Colors,
    game_states::GameState,
    overlays::overlay_manager::{OverlayData, OverlayManager},
    sprite_manager::SpriteManager,
    sprites::sprite::CanvasDimension,
};

const LIFES_WHEN_STARTING: u32 = 3;
const ASTEROIDS_START_MAX: u32 = 12;
const ASTEROIDS_LEVEL_INCREMENT: u32 = 3;
const GAME_OVER_STATE_TIMEOUT: u32 = 8000; // ms
const ADD_LIFE_WHEN_SCORED: u32 = 3000;
const ASTEROID_HIT_SCORE: u32 = 10;
const UFO_HIT_SCORE: u32 = 50;
const UFO_INIT_FREQUENCY: u32 = 25000; // ms
const UFO_DECREMENT_FREQUENCY: u32 = 1000; // ms
const UFO_MINIMAL_FREQUENCY: u32 = 10000; // ms
const UFO_SHOOT_INIT_FREQUENCY: u32 = 15000; // ms
const UFO_SHOOT_DECREMENT_FREQUENCY: u32 = 500; // ms
const UFO_SHOOT_MINIMAL_FREQUENCY: u32 = 5000; // ms

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
    ufo_frequency: u32,
    ufo_shoot_frequency: u32,
    life_added_so_far: u32,
    canvas: CanvasDimension,
}

impl GameManager {
    pub fn new(canvas: CanvasDimension) -> GameManager {
        GameManager {
            game_state: GameState::Homescreen,
            sprite_manager: SpriteManager::new(canvas),
            overlay_manager: OverlayManager::new(canvas),
            is_game_paused: false,
            level: 0,
            lifes: 0,
            top_score: 0,
            score: 0,
            max_asteroids: 0,
            ufo_frequency: 0,
            ufo_shoot_frequency: 0,
            life_added_so_far: 0,
            canvas,
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
    }

    pub fn draw(&mut self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        // Clear canvas
        canvas.set_stroke_style(&Colors::Edge.value().into());
        canvas.set_fill_style(&Colors::Background.value().into());

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
        // clearTimeout(this.gameOverTimeout);

        self.score = 0;
        self.level = 1;

        self.score = 0;
        self.level = 1;
        self.max_asteroids = ASTEROIDS_START_MAX;
        self.lifes = LIFES_WHEN_STARTING;

        self.overlay_manager.set_life_count(self.lifes - 1);

        self.life_added_so_far = 0;
        self.ufo_frequency = UFO_INIT_FREQUENCY;
        self.ufo_shoot_frequency = UFO_SHOOT_INIT_FREQUENCY;

        self.sprite_manager.reset();

        self.start_level();
    }

    fn start_level(&mut self) {
        self.game_state = GameState::Playing;

        self.sprite_manager.start_level(
            self.max_asteroids,
            self.ufo_frequency,
            self.ufo_shoot_frequency,
        );
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.max_asteroids += ASTEROIDS_LEVEL_INCREMENT;
        self.ufo_frequency -= UFO_DECREMENT_FREQUENCY;
        self.ufo_shoot_frequency -= UFO_SHOOT_DECREMENT_FREQUENCY;

        if self.ufo_frequency < UFO_MINIMAL_FREQUENCY {
            self.ufo_frequency = UFO_MINIMAL_FREQUENCY;
        }

        if self.ufo_shoot_frequency < UFO_SHOOT_MINIMAL_FREQUENCY {
            self.ufo_shoot_frequency = UFO_SHOOT_MINIMAL_FREQUENCY;
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
            self.sprite_manager.stop_level();

            if self.lifes == 0 {
                self.game_state = GameState::GameOver;
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
