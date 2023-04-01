use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::{colors::Colors, interval::Interval, vector::Vector};

use super::{
    laser::Laser,
    sprite::{CanvasDimension, Spritable, Sprite, SpriteData},
};

const UFO_WIDTH: f64 = 60.0;
const UFO_HEIHT: f64 = 25.0;
const CHANGE_HEADING_FREQUENCY: u32 = 6000;
const VARIABILITY_IN_HEADING: u32 = 3000;
const VARIABILITY_IN_SHOOTING: u32 = 1000;

pub struct Ufo {
    pub sprite: Sprite,
    pub ufo_shoot_frequency: u32,
    lasers: Vec<Laser>,
    ship_position: Vector,
    vertices: Vec<Vector>,
    change_heading_interval: Interval,
    shoot_interval: Interval,
}

impl Spritable for Ufo {
    fn update(&mut self) {
        self.sprite.update();

        // self.heading += self.sprite.sprite_data.rotation;

        // if self.is_boosting {
        //     let mut force = Vector::from_angle(self.heading);
        //     force.limit(0.15);
        //     self.sprite.sprite_data.velocity += force;
        // }

        // self.sprite.sprite_data.velocity.limit(10.0);
        // self.sprite.sprite_data.velocity.mult(0.995);

        // for laser in &mut self.lasers {
        //     laser.update();
        // }

        // self.lasers = self
        //     .lasers
        //     .drain(..)
        //     .filter(|laser| !laser.sprite.is_offscreen)
        //     .collect();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        let mut sprite_data = self.sprite.sprite_data;

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
    pub fn new(sprite_data: SpriteData, ufo_shoot_frequency: u32, canvas: CanvasDimension) -> Ufo {
        Ufo {
            sprite: Sprite::new(sprite_data, canvas),
            ufo_shoot_frequency,
            lasers: Vec::new(),
            ship_position: Vector::new(0.0, 0.0),
            vertices: Ufo::generate_vertices(),
            shoot_interval: Interval::new(),
            change_heading_interval: Interval::new(),
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

    pub fn set_ship_position(&mut self, position: Vector) {}

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
