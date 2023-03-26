use std::f64::consts::PI;

use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::vector::Vector;

use super::sprite::{Canvas, Spritable, Sprite, SpriteData};

pub struct Potatoid {
    sides: u8,
    sprite: Sprite,
    vertices: Vec<Vector>,
}

impl Spritable for Potatoid {
    fn update(&mut self) {
        self.sprite.update();
    }

    fn draw(&self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        canvas.set_stroke_style(&"#0000ff".into());

        let position = self.sprite.sprite_data.position;
        let _result = canvas.translate(position.x, position.y);
        let _result = canvas.rotate(self.sprite.sprite_data.rotation);
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

    fn check_window_edges(&mut self) -> bool {
        false
    }

    fn collide_width(&self) -> bool {
        false
    }
}

impl Potatoid {
    pub fn new(sprite_data: SpriteData, sides: u8, canvas: Canvas) -> Potatoid {
        let mut potatoid = Potatoid {
            sides,
            sprite: Sprite::new(sprite_data, canvas),
            vertices: Vec::new(),
        };

        potatoid.generate_vertices();

        potatoid
    }

    fn generate_vertices(&mut self) {
        for side in 0..self.sides {
            let diameter = self.sprite.sprite_data.diameter;
            let radius = rand::thread_rng().gen_range(diameter * 0.35..diameter * 0.5);
            let angle = 2.0 * PI / self.sides as f64 * side as f64;
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            self.vertices.push(Vector::new(x, y));
        }
    }
}
