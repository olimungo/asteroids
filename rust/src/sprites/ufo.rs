use web_sys::CanvasRenderingContext2d;

use crate::utils::{general::random, interval::Interval, vector::Vector};

use super::{
    laser::Laser,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

const UFO_WIDTH: f64 = 60.0;
const UFO_HEIHT: f64 = 25.0;
const UFO_VELOCITY: f64 = 2.0;
const CHANGE_HEADING_FREQUENCY: u32 = 6000;
const VARIABILITY_IN_HEADING: u32 = 3000;
const VARIABILITY_IN_SHOOTING: u32 = 1000;

pub struct Ufo {
    pub sprite: Sprite,
    pub shoot_frequency: u32,
    lasers: Vec<Laser>,
    ship_position: Vector,
    vertices: Vec<Vector>,
    heading_interval: Interval,
    shoot_interval: Interval,
}

impl Spritable for Ufo {
    fn update(&mut self) {
        if self.heading_interval.is_ellapsed() {
            self.sprite.data.velocity = Vector::random(-UFO_VELOCITY, UFO_VELOCITY);
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
            sprite_data.position.x - UFO_WIDTH / 2.0,
            sprite_data.position.y - UFO_HEIHT / 2.0,
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
    pub fn new(sprite_data: SpriteData, shoot_frequency: u32, canvas: CanvasDimension) -> Ufo {
        let mut shoot_interval = Interval::new();
        let mut heading_interval = Interval::new();

        let random_interval = random(
            CHANGE_HEADING_FREQUENCY - VARIABILITY_IN_HEADING,
            CHANGE_HEADING_FREQUENCY + VARIABILITY_IN_HEADING,
        );

        heading_interval.set(random_interval);

        if shoot_frequency > 0 {
            let random_interval = random(
                shoot_frequency - VARIABILITY_IN_SHOOTING,
                shoot_frequency + VARIABILITY_IN_SHOOTING,
            );

            shoot_interval.set(random_interval);
        }

        let mut new_sprite_data = sprite_data;
        new_sprite_data.diameter = (UFO_WIDTH + UFO_HEIHT) / 2.0;

        Ufo {
            sprite: Sprite::new(new_sprite_data, canvas),
            shoot_frequency,
            lasers: Vec::new(),
            ship_position: Vector::new(0.0, 0.0),
            vertices: Ufo::generate_vertices(),
            shoot_interval,
            heading_interval,
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

    fn generate_vertices() -> Vec<Vector> {
        let mut vertices = Vec::new();

        let height1 = UFO_HEIHT / 4.0;
        let height2 = (UFO_HEIHT / 8.0) * 5.0;
        let width1 = UFO_WIDTH / 3.0;
        let width2 = UFO_WIDTH * 0.66;
        let width3 = (UFO_WIDTH / 10.0) * 6.0;
        let width4 = (UFO_WIDTH / 10.0) * 4.0;

        vertices.push(Vector::new(7.0, height2));
        vertices.push(Vector::new(width1, UFO_HEIHT - 1.0));
        vertices.push(Vector::new(width2, UFO_HEIHT - 1.0));
        vertices.push(Vector::new(UFO_WIDTH - 7.0, height2));
        vertices.push(Vector::new(7.0, height2));
        vertices.push(Vector::new(width1, height1));
        vertices.push(Vector::new(width2, height1));
        vertices.push(Vector::new(UFO_WIDTH - 7.0, height2));
        vertices.push(Vector::new(width2, height1));
        vertices.push(Vector::new(width3, 1.0));
        vertices.push(Vector::new(width4, 1.0));
        vertices.push(Vector::new(width1, height1));

        vertices
    }
}
