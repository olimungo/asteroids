use std::f64::consts::PI;

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
    ship_fragments: Vec<Potatoid>,
    pub is_ship_active: bool,
    ufo_create_frequency: u32,
    pub count_asteroids_hit: u32,
    pub count_ufo_hit: u32,
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
            ship_fragments: Vec::new(),
            is_ship_active: false,
            ufo_create_frequency: 0,
            count_asteroids_hit: 0,
            count_ufo_hit: 0,
            canvas,
        };

        sprite_manager.create_asteroids(20);

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
        if self.is_ship_active {
            self.ship.update();
        }

        if !self.ship_fragments.is_empty() {
            for index in 0..self.ship_fragments.len() {
                self.ship_fragments[index].update();
            }
        }

        let mut new_asteroids = Vec::new();

        for index in (0..self.asteroids.len()).rev() {
            self.asteroids[index].update();

            if self.ship.lasers_collide_with(self.asteroids[index].sprite) {
                new_asteroids.extend(self.asteroids[index].break_up());

                self.asteroids.remove(index);

                self.count_asteroids_hit += 1;
            } else if self.is_ship_active && self.ship.collide_with(self.asteroids[index].sprite) {
                self.is_ship_active = false;
                self.ship_fragments = self.ship.break_up();
            }
        }

        self.asteroids.extend(new_asteroids);
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        if self.is_ship_active {
            self.ship.draw(canvas.clone());
        }

        if !self.ship_fragments.is_empty() {
            for index in 0..self.ship_fragments.len() {
                self.ship_fragments[index].draw(canvas.clone());
            }
        }

        for counter in 0..self.asteroids.len() {
            self.asteroids[counter].draw(canvas.clone());
        }
    }

    pub fn create_asteroids(&mut self, count: u32) {
        self.asteroids.clear();

        for counter in 0..count {
            let radius = rand::thread_rng()
                .gen_range(self.canvas.height / 2.0..(self.canvas.height / 2.0) * 1.3);

            let angle = 2.0 * PI / count as f64 * counter as f64;

            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let mut position = Vector::new(x, y);

            position += Vector::new(self.canvas.width / 2.0, self.canvas.height / 2.0);

            let velocity = Vector::random();

            let diameter = rand::thread_rng().gen_range(40.0..120.0);

            let rotation = 0.0;

            let randomness = rand::thread_rng().gen_range(0..10);

            let rotation_step = match randomness {
                x if x < 5 => 0.01,
                _ => -0.01,
            };

            let sides = rand::thread_rng().gen_range(8..20);

            let sprite_data = SpriteData {
                position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            };

            self.asteroids
                .push(Potatoid::new(sprite_data, sides, self.canvas));
        }
    }

    pub fn get_asteroids_count(&self) -> usize {
        self.asteroids.len()
    }

    pub fn pause(&self) {
        // todo!("pause sprite manager");
    }

    pub fn unpause(&self) {
        // todo!("unpause sprite manager");
    }

    pub fn start_level(
        &mut self,
        count_asteroids: u32,
        ufo_create_frequency: u32,
        ufo_shoot_frequency: u32,
    ) {
        self.ufo_create_frequency = ufo_shoot_frequency;

        self.reset();

        self.ship.sprite.sprite_data.position =
            Vector::new(self.canvas.width / 2.0, self.canvas.height / 2.0);
        self.ship.sprite.sprite_data.velocity = Vector::new(0.0, 0.0);
        self.ship.heading = -PI / 2.0;
        self.is_ship_active = true;

        self.create_asteroids(count_asteroids);

        // this.createUfoInterval = new Interval(
        //     this.p5.random(
        //         ufoCreateFrequency - VARIABILITY_IN_CREATING_UFOS,
        //         ufoCreateFrequency + VARIABILITY_IN_CREATING_UFOS
        //     )
        // );
    }

    pub fn stop_level(&mut self) {
        // this.createUfoInterval = null;
        // this.ufos = [];
        // this.ufoShootFrequency = 0;

        self.count_asteroids_hit = 0;
        self.count_ufo_hit = 0;
    }

    pub fn reset(&mut self) {
        self.asteroids.clear();
        self.ship_fragments.clear();
        // this.ufos = [];
    }
}
