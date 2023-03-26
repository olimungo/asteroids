use crate::sprites::sprite::Spritable;

pub struct BasicSprite {}

impl Spritable for BasicSprite {
    fn update(&mut self) {}

    fn draw(&self) {}

    fn check_window_edges(&self) -> bool {
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
}
