use web_sys::CanvasRenderingContext2d;

use crate::utils::{config::Config, general::random, interval::Interval, vector::Vector};

use super::{
    laser::Laser,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

pub struct Ufo {
    pub sprite: Sprite,
    pub shoot_frequency: u32,
    lasers: Vec<Laser>,
    ship_position: Vector,
    vertices: Vec<Vector>,
    heading_interval: Interval,
    shoot_interval: Interval,
    config: Config,
}

impl Spritable for Ufo {
    fn update(&mut self) {
        if self.heading_interval.is_ellapsed() {
            self.sprite.data.velocity = Vector::random(
                -self.config.sprites.ufo.ufo_velocity,
                self.config.sprites.ufo.ufo_velocity,
            );
        }

        if self.shoot_interval.is_ellapsed() {
            self.shoot();
        }

        for laser in &mut self.lasers {
            laser.update();
        }

        self.lasers = self
            .lasers
            .drain(..)
            .filter(|laser| !laser.sprite.is_offscreen)
            .collect();

        self.sprite.update();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        let sprite_data = self.sprite.data;

        for laser in &self.lasers {
            laser.draw(canvas.clone());
        }

        canvas.save();

        let _result = canvas.translate(
            sprite_data.position.x - self.config.sprites.ufo.ufo_width / 2.0,
            sprite_data.position.y - self.config.sprites.ufo.ufo_height / 2.0,
        );

        canvas.begin_path();

        let vertices = &self.vertices;

        canvas.move_to(vertices[0].x, vertices[0].y);

        for vertex in vertices {
            canvas.line_to(vertex.x, vertex.y);
        }

        canvas.close_path();

        canvas.stroke();

        canvas.restore();
    }

    fn collide_with(&self, sprite: Sprite) -> bool {
        self.sprite.collide_with(sprite)
    }
}

impl Ufo {
    pub fn new(shoot_frequency: u32, canvas: CanvasDimension) -> Ufo {
        let config = Config::new();

        let random_corner = random(1, 5);
        let mut position = Vector::new(canvas.width / 2.0, canvas.height / 2.0);

        match random_corner {
            1 => position.x += canvas.width / 2.0,
            2 => position.x -= canvas.width / 2.0,
            3 => position.y += canvas.height / 2.0,
            _ => position.y -= canvas.height / 2.0,
        }

        let mut heading_interval = Interval::new();
        let mut shoot_interval = Interval::new();

        let random_interval = random(
            config.sprites.ufo.change_heading_frequency - config.sprites.ufo.variability_in_heading,
            config.sprites.ufo.change_heading_frequency + config.sprites.ufo.variability_in_heading,
        );

        heading_interval.set(random_interval);

        if shoot_frequency > 0 {
            let random_interval = random(
                shoot_frequency - config.sprites.ufo.variability_in_shooting,
                shoot_frequency + config.sprites.ufo.variability_in_shooting,
            );

            shoot_interval.set(random_interval);
        }

        let sprite_data = SpriteData {
            position,
            velocity: Vector::random(-2.0, 2.0),
            diameter: (config.sprites.ufo.ufo_width + config.sprites.ufo.ufo_height) / 2.0,
            rotation: 0.0,
            rotation_step: 0.0,
        };

        Ufo {
            sprite: Sprite::new(sprite_data, canvas),
            shoot_frequency,
            lasers: Vec::new(),
            ship_position: Vector::new(0.0, 0.0),
            vertices: Ufo::generate_vertices(&config),
            shoot_interval,
            heading_interval,
            config,
        }
    }

    pub fn lasers_collide_with(&mut self, sprite: Sprite) -> bool {
        for (index, laser) in &mut self.lasers.iter().enumerate() {
            if laser.collide_with(sprite) {
                self.lasers.remove(index);
                return true;
            }
        }

        false
    }

    pub fn set_ship_position(&mut self, position: Vector) {
        self.ship_position = position;
    }

    pub fn shoot(&mut self) {
        let mut velocity = self.ship_position;
        velocity -= self.sprite.data.position;
        velocity.limit(7.0);

        let diameter = 2.0;
        let rotation = 0.0;
        let rotation_step = 0.0;

        let laser = Laser::new(
            SpriteData {
                position: self.sprite.data.position,
                velocity,
                diameter,
                rotation,
                rotation_step,
            },
            self.sprite.canvas,
        );

        self.lasers.push(laser);
    }

    pub fn pause(&mut self) {
        self.heading_interval.pause();
        self.shoot_interval.pause();
    }

    pub fn unpause(&mut self) {
        self.heading_interval.unpause();
        self.shoot_interval.unpause();
    }

    fn generate_vertices(config: &Config) -> Vec<Vector> {
        let mut vertices = Vec::new();

        let height1 = config.sprites.ufo.ufo_height / 4.0;
        let height2 = (config.sprites.ufo.ufo_height / 8.0) * 5.0;
        let width1 = config.sprites.ufo.ufo_width / 3.0;
        let width2 = config.sprites.ufo.ufo_width * 0.66;
        let width3 = (config.sprites.ufo.ufo_width / 10.0) * 6.0;
        let width4 = (config.sprites.ufo.ufo_width / 10.0) * 4.0;

        vertices.push(Vector::new(7.0, height2));
        vertices.push(Vector::new(width1, config.sprites.ufo.ufo_height - 1.0));
        vertices.push(Vector::new(width2, config.sprites.ufo.ufo_height - 1.0));
        vertices.push(Vector::new(config.sprites.ufo.ufo_width - 7.0, height2));
        vertices.push(Vector::new(7.0, height2));
        vertices.push(Vector::new(width1, height1));
        vertices.push(Vector::new(width2, height1));
        vertices.push(Vector::new(config.sprites.ufo.ufo_width - 7.0, height2));
        vertices.push(Vector::new(width2, height1));
        vertices.push(Vector::new(width3, 1.0));
        vertices.push(Vector::new(width4, 1.0));
        vertices.push(Vector::new(width1, height1));

        vertices
    }
}
