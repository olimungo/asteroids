use std::f64::consts::PI;

use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::{
    sprites::{
        explosion::Explosion,
        potatoid::Potatoid,
        ship::Ship,
        sprite::{CanvasDimension, Spritable, SpriteData},
        ufo::Ufo,
    },
    utils::{config::Config, interval::Interval, vector::Vector},
};

pub struct SpriteManager {
    asteroids: Vec<Potatoid>,
    ship: Ship,
    ship_fragments: Vec<Potatoid>,
    ufos: Vec<Ufo>,
    explosions: Vec<Explosion>,
    pub is_ship_active: bool,
    pub count_asteroids_hit: u32,
    pub count_ufo_hit: u32,
    create_ufo_interval: Interval,
    ufo_shoot_frequency: u32,
    canvas: CanvasDimension,
    config: Config,
}

impl SpriteManager {
    pub fn new(canvas: CanvasDimension) -> SpriteManager {
        let position = Vector::new(canvas.width / 2.0, canvas.height / 2.0);
        let velocity = Vector::new(0.0, 0.0);
        let diameter = 0.0;
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
            false,
            canvas,
        );

        SpriteManager {
            asteroids: Vec::new(),
            ship,
            ship_fragments: Vec::new(),
            ufos: Vec::new(),
            explosions: Vec::new(),
            is_ship_active: false,
            count_asteroids_hit: 0,
            count_ufo_hit: 0,
            create_ufo_interval: Interval::new(),
            ufo_shoot_frequency: 0,
            canvas,
            config: Config::new(),
        }
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

        for index in (0..self.asteroids.len()).rev() {
            self.asteroids[index].update();

            if self.ship.lasers_collide_with(self.asteroids[index].sprite) {
                let new_asteroids = self.asteroids[index].break_up();

                if new_asteroids.is_empty() {
                    let explosion = Explosion::new(self.asteroids[index].sprite.data, self.canvas);

                    self.explosions.push(explosion);
                } else {
                    self.asteroids.extend(new_asteroids);
                }

                self.asteroids.remove(index);

                self.count_asteroids_hit += 1;
            } else if self.is_ship_active && self.ship.collide_with(self.asteroids[index].sprite) {
                self.is_ship_active = false;
                self.ship_fragments = self.ship.break_up();
            }
        }

        for index in (0..self.ufos.len()).rev() {
            self.ufos[index].set_ship_position(self.ship.sprite.data.position);
            self.ufos[index].update();

            if self.is_ship_active {
                if self.ship.lasers_collide_with(self.ufos[index].sprite) {
                    self.count_ufo_hit += 1;

                    let explosion = Explosion::new(self.ufos[index].sprite.data, self.canvas);

                    self.explosions.push(explosion);

                    self.ufos.remove(index);
                } else if self.ship.collide_with(self.ufos[index].sprite)
                    || self.ufos[index].lasers_collide_with(self.ship.sprite)
                {
                    self.ship_fragments = self.ship.break_up();
                    self.is_ship_active = false;
                }
            }
        }

        if self.create_ufo_interval.is_ellapsed() {
            self.create_ufo(self.ufo_shoot_frequency);
        }

        for index in (0..self.explosions.len()).rev() {
            self.explosions[index].update();

            if self.explosions[index].is_faded {
                self.explosions.remove(index);
            }
        }
    }

    pub fn draw(&self, canvas: CanvasRenderingContext2d) {
        for explosion in &self.explosions {
            explosion.draw(canvas.clone());
        }

        if self.is_ship_active {
            self.ship.draw(canvas.clone());
        }

        if !self.ufos.is_empty() {
            for index in 0..self.ufos.len() {
                self.ufos[index].draw(canvas.clone());
            }
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

            let velocity = Vector::random_limit(
                self.config.sprites.manager.asteroid_velocity_limit,
                self.config.sprites.manager.asteroid_velocity_min_value,
            );

            let diameter = rand::thread_rng().gen_range(
                self.config.sprites.manager.asteroid_diameter_min
                    ..self.config.sprites.manager.asteroid_diameter_max,
            );

            let rotation = 0.0;

            let randomness = rand::thread_rng().gen_range(0.0..1.0);

            let rotation_step = match randomness {
                x if x < 0.5 => 0.01,
                _ => -0.01,
            };

            let sides = rand::thread_rng().gen_range(
                self.config.sprites.manager.asteroid_sides_min
                    ..self.config.sprites.manager.asteroid_sides_max,
            );

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

    pub fn create_ufo(&mut self, ufo_shoot_frequency: u32) {
        self.ufos.push(Ufo::new(ufo_shoot_frequency, self.canvas));
    }

    pub fn get_asteroids_count(&self) -> usize {
        self.asteroids.len()
    }

    pub fn start_level(
        &mut self,
        count_asteroids: u32,
        ufo_create_frequency: u32,
        ufo_shoot_frequency: u32,
    ) {
        self.ufo_shoot_frequency = ufo_shoot_frequency;

        self.reset();

        self.ship.sprite.data.position =
            Vector::new(self.canvas.width / 2.0, self.canvas.height / 2.0);
        self.ship.sprite.data.velocity = Vector::new(0.0, 0.0);
        self.ship.heading = -PI / 2.0;
        self.is_ship_active = true;

        self.create_asteroids(count_asteroids);

        self.create_ufo_interval.set(ufo_create_frequency);
    }

    pub fn stop_level(&mut self) {
        self.ufos.clear();
        self.create_ufo_interval.cancel();
        self.ufo_shoot_frequency = 0;
        self.count_asteroids_hit = 0;
        self.count_ufo_hit = 0;
    }

    pub fn reset(&mut self) {
        self.asteroids.clear();
        self.ship_fragments.clear();
        self.ufos.clear();
    }

    pub fn pause(&mut self) {
        self.create_ufo_interval.pause();

        for index in 0..self.ufos.len() {
            self.ufos[index].pause();
        }
    }

    pub fn unpause(&mut self) {
        self.create_ufo_interval.unpause();

        for index in 0..self.ufos.len() {
            self.ufos[index].unpause();
        }
    }
}
