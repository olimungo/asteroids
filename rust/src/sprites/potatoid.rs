use std::f64::consts::PI;

use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::utils::{config::Config, vector::Vector};

use super::sprite::{CanvasDimension, Spritable, Sprite, SpriteData};

pub struct Potatoid {
    sides: u32,
    pub sprite: Sprite,
    vertices: Vec<Vector>,
    config: Config,
}

impl Spritable for Potatoid {
    fn update(&mut self) {
        self.sprite.update();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let position = self.sprite.data.position;
        let _result = canvas.translate(position.x, position.y);
        let _result = canvas.rotate(self.sprite.data.rotation);
        let vertices = &self.vertices;

        canvas.begin_path();

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

impl Potatoid {
    pub fn new(sprite_data: SpriteData, sides: u32, canvas: CanvasDimension) -> Potatoid {
        let config = Config::new();

        let mut potatoid = Potatoid {
            sides,
            sprite: Sprite::new(sprite_data, canvas),
            vertices: Vec::new(),
            config,
        };

        potatoid.generate_vertices();

        potatoid
    }

    fn generate_vertices(&mut self) {
        for side in 0..self.sides {
            let diameter = self.sprite.data.diameter;
            let radius = rand::thread_rng().gen_range(
                diameter * self.config.sprites.potatoid.vertex_radius_min
                    ..diameter * self.config.sprites.potatoid.vertex_radius_max,
            );
            let angle = 2.0 * PI / self.sides as f64 * side as f64;
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            self.vertices.push(Vector::new(x, y));
        }
    }

    pub fn break_up(&self) -> Vec<Potatoid> {
        let mut new_asteroids = Vec::new();
        let diameter = self.sprite.data.diameter;

        if diameter
            > self
                .config
                .sprites
                .potatoid
                .potatoid_minimal_diameter_breakup
        {
            let count_new_potatoids = match diameter {
                x if x > self.config.sprites.potatoid.diameter_max => 3,
                _ => 2,
            };

            for _counter in 0..count_new_potatoids {
                let mut sprite_data = self.sprite.data;

                sprite_data.diameter /= count_new_potatoids as f64 * 0.80;
                sprite_data.velocity = Vector::random_limit(1.2, 0.8);

                new_asteroids.push(Potatoid::new(sprite_data, self.sides, self.sprite.canvas));
            }
        }

        new_asteroids
    }
}
