use crate::sprites::sprite::Spritable;

pub struct Potatoid {}

impl Spritable for Potatoid {
    fn update(&mut self) {}

    fn draw(&self) {}

    fn check_window_edges(&self) -> bool {
        false
    }

    fn collide_width(&self) -> bool {
        false
    }
}

impl Potatoid {
    pub fn new() -> Potatoid {
        Potatoid {}
    }
}
