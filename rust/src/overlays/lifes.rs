use web_sys::CanvasRenderingContext2d;

use crate::{
    sprites::{
        ship::Ship,
        sprite::{CanvasDimension, Spritable, SpriteData},
    },
    vector::Vector,
};

pub struct Lifes {
    ships: Vec<Ship>,
    canvas: CanvasDimension,
    center_x: f64,
    center_y: f64,
}

impl Lifes {
    pub fn new(canvas: CanvasDimension) -> Lifes {
        Lifes {
            ships: Vec::new(),
            canvas,
            center_x: canvas.width / 2.0,
            center_y: canvas.height / 2.0,
        }
    }
}

impl Lifes {
    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        for ship in &self.ships {
            ship.draw(canvas.clone());
        }

        canvas.restore();
    }

    pub fn set_life_count(&mut self, count: u32) {
        let y = 50.0;
        let mut x = 50.0;

        self.ships.clear();

        for _counter in 0..count {
            let position = Vector::new(x, y);
            let velocity = Vector::new(0.0, 0.0);
            let diameter = 36.0;
            let rotation = 0.0;
            let rotation_step = 0.0;

            let sprite_data = SpriteData {
                position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            };

            self.ships.push(Ship::new(sprite_data, true, self.canvas));

            x += 40.0;
        }
    }
}
