use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, sprite_manager::SpriteManager, sprites::sprite::CanvasDimension};

pub struct GameManager {
    sprite_manager: SpriteManager,
    canvas: CanvasDimension,
}

impl GameManager {
    pub fn new(canvas: CanvasDimension) -> GameManager {
        GameManager {
            sprite_manager: SpriteManager::new(canvas),
            canvas,
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        self.sprite_manager.key_pressed(key);
    }

    pub fn key_released(&mut self, key: &str) {
        self.sprite_manager.key_released(key);
    }

    pub fn update(&mut self) {
        self.sprite_manager.update();
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.set_stroke_style(&Colors::Edge.value().into());
        canvas.set_fill_style(&Colors::Edge.value().into());

        canvas.clear_rect(0f64, 0f64, self.canvas.width, self.canvas.height);

        self.sprite_manager.draw(canvas);
    }
}
