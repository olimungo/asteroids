use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::Spritable;

use super::sprite::Sprite;

pub struct BasicSprite {}

impl Spritable for BasicSprite {
    fn update(&mut self) {}

    fn draw(&self, canvas: CanvasRenderingContext2d) {}

    fn check_window_edges(&mut self) -> bool {
        false
    }

    fn collide_width(&self) -> bool {
        false
    }
}

impl BasicSprite {
    pub fn new() -> BasicSprite {
        BasicSprite {}
    }

    pub fn test(mut sprite: Sprite<BasicSprite>) {}
}
