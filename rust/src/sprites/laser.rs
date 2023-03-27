use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use super::sprite::{CanvasDimension, Spritable, Sprite, SpriteData};

pub struct Laser {
    pub sprite: Sprite,
}

impl Spritable for Laser {
    fn update(&mut self) {
        self.sprite.update();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let _result = canvas.translate(
            self.sprite.sprite_data.position.x,
            self.sprite.sprite_data.position.y,
        );

        canvas.begin_path();

        let _result = canvas.arc(0.0, 0.0, self.sprite.sprite_data.diameter, 0.0, 2.0 * PI);

        canvas.fill();

        canvas.restore();
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        self.sprite.collide_with(sprite)
    }
}

impl Laser {
    pub fn new(sprite_data: SpriteData, canvas: CanvasDimension) -> Laser {
        Laser {
            sprite: Sprite::new(sprite_data, canvas),
        }
    }
}
