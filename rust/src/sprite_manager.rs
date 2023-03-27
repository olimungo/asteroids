use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::{
    sprites::{
        potatoid::Potatoid,
        ship::Ship,
        sprite::{CanvasDimension, Spritable, SpriteData},
    },
    vector::Vector,
};

pub struct SpriteManager {
    asteroids: Vec<Potatoid>,
    ship: Ship,
    canvas: CanvasDimension,
}

impl SpriteManager {
    pub fn new(canvas: CanvasDimension) -> SpriteManager {
        let position = Vector::new(canvas.width / 2.0, canvas.height / 2.0);
        let velocity = Vector::new(0.0, 0.0);
        let diameter = 36.0;
        let rotation = 0.0;
        let rotation_step = 0.0;

        let ship = Ship::new(
            SpriteData {
                position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            },
            canvas,
        );

        let mut sprite_manager = SpriteManager {
            asteroids: Vec::new(),
            ship,
            canvas,
        };

        sprite_manager.create_asteroids(5);

        sprite_manager
    }

    pub fn key_pressed(&mut self, key: &str) {
        match key {
            "ArrowUp" => self.ship.set_boost(true),
            "ArrowLeft" => self.ship.set_rotation(-0.1),
            "ArrowRight" => self.ship.set_rotation(0.1),
            " " => self.ship.shoot(),
            _ => {}
        }
    }

    pub fn key_released(&mut self, key: &str) {
        match key {
            "ArrowUp" => self.ship.set_boost(false),
            "ArrowLeft" => self.ship.set_rotation(0.0),
            "ArrowRight" => self.ship.set_rotation(0.0),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.ship.update();

        for index in (0..self.asteroids.len()).rev() {
            self.asteroids[index].update();

            if self.ship.lasers_collide_with(self.asteroids[index].sprite) {
                self.asteroids.remove(index);
            }
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        self.ship.draw(canvas.clone());

        for counter in 0..self.asteroids.len() {
            self.asteroids[counter].draw(canvas.clone());
        }
    }

    pub fn create_asteroids(&mut self, count: u32) {
        self.asteroids.clear();

        for _counter in 0..count {
            let mut position = Vector::random();
            position += Vector::new(self.canvas.width / 2.0, self.canvas.height / 2.0);

            let velocity = Vector::random();
            let diameter = rand::thread_rng().gen_range(50.0..100.0);
            let rotation = 0.0;
            let rotation_step = rand::thread_rng().gen_range(-0.05..0.05);

            let sprite_data = SpriteData {
                position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            };

            self.asteroids
                .push(Potatoid::new(sprite_data, 12u8, self.canvas));
        }

        // for (let counter = 0; counter < count; counter++) {
        //     const radius = this.p5.random(
        //         this.p5.height / 2,
        //         (this.p5.height / 2) * 1.3
        //     );

        //     const angle = this.p5.map(counter, 0, count, 0, this.p5.TWO_PI);
        //     const x = radius * this.p5.cos(angle);
        //     const y = radius * this.p5.sin(angle);
        //     const position = new P5.Vector(x, y);

        //     position.add(this.p5.width / 2, this.p5.height / 2);

        //     const diameter = this.p5.random(DIAMETER_MIN, DIAMETER_MAX);

        //     const rotationStep = this.p5.map(
        //         this.p5.random(1),
        //         0,
        //         1,
        //         -0.01,
        //         0.01
        //     );

        //     const sides = this.p5.floor(this.p5.random(SIDES_MIN, SIDES_MAX));

        //     this.asteroids.push(
        //         new Patatoid(
        //             this.p5,
        //             position,
        //             diameter,
        //             P5.Vector.random2D(),
        //             rotationStep,
        //             sides
        //         )
        //     );
        // }
    }
}
