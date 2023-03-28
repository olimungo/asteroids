use web_sys::CanvasRenderingContext2d;

use crate::{
    colors::Colors, game_states::GameState, log, overlays::overlay_manager::OverlayManager,
    sprite_manager::SpriteManager, sprites::sprite::CanvasDimension,
};

pub struct GameManager {
    game_state: GameState,
    sprite_manager: SpriteManager,
    overlay_manager: OverlayManager,
    is_game_paused: bool,
    score: u32,
    level: u16,
    canvas: CanvasDimension,
}

impl GameManager {
    pub fn new(canvas: CanvasDimension) -> GameManager {
        GameManager {
            game_state: GameState::Homescreen,
            sprite_manager: SpriteManager::new(canvas),
            overlay_manager: OverlayManager::new(canvas),
            is_game_paused: false,
            score: 0,
            level: 0,
            canvas,
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        self.sprite_manager.key_pressed(key);
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
            todo!("next level");
            // self.next_level();
        }

        if self.game_state == GameState::NextLife && key == "s" {
            todo!("next life");
            // self.next_life();
        }
    }

    pub fn key_released(&mut self, key: &str) {
        if self.game_state == GameState::Playing {
            self.sprite_manager.key_released(key);
        }
    }

    pub fn update(&mut self) {
        self.sprite_manager.update();
        self.overlay_manager.update(self.game_state);
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&Colors::Edge.value().into());
        canvas.set_fill_style(&Colors::Background.value().into());

        canvas.fill_rect(0f64, 0f64, self.canvas.width, self.canvas.height);

        canvas.set_fill_style(&Colors::Edge.value().into());

        self.overlay_manager.draw_background(canvas.clone());

        self.sprite_manager.draw(canvas.clone());

        self.overlay_manager
            .draw_foreground(self.game_state, canvas.clone());

        canvas.restore();
    }

    fn start_game(&mut self) {
        // clearTimeout(this.gameOverTimeout);

        self.score = 0;
        self.level = 1;

        // this.score = 0;
        // this.level = 1;
        // this.maxAsteroids = ASTEROIDS_START_MAX;
        // this.lifes = LIFES_WHEN_STARTING;
        // this.overlaysManager.setLifeCount(this.lifes - 1);
        // this.lifeAddedSoFar = 0;
        // this.ufoFrequency = UFO_INIT_FREQUENCY;
        // this.ufoShootFrequency = UFO_SHOOT_INIT_FREQUENCY;

        // this.spritesManager.reset();

        self.sprite_manager.reset();

        self.start_level();
    }

    fn start_level(&mut self) {
        self.game_state = GameState::Playing;

        self.sprite_manager.start_level(20, 1000, 1000);

        // this.spritesManager.startLevel(
        //     this.maxAsteroids,
        //     this.ufoFrequency,
        //     this.ufoShootFrequency
        // );
    }
}
