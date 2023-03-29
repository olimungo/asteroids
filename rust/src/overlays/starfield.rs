use web_sys::CanvasRenderingContext2d;

use crate::sprites::sprite::CanvasDimension;

use super::star::Star;

const STARS_COUNT: u32 = 400;

pub struct Starfield {
    stars: Vec<Star>,
    canvas: CanvasDimension,
}

impl Starfield {
    pub fn new(canvas: CanvasDimension) -> Starfield {
        let mut stars = Vec::<Star>::new();

        for _counter in 0..STARS_COUNT {
            stars.push(Star::new(canvas));
        }

        Starfield { stars, canvas }
    }
}

impl Starfield {
    pub fn update(&mut self) {
        for star in &mut self.stars {
            star.update();
        }
    }

    pub fn draw(&mut self, canvas: CanvasRenderingContext2d) {
        canvas.save();

        let _result = canvas.translate(self.canvas.width / 2.0, self.canvas.height / 2.0);

        for star in &mut self.stars {
            star.draw(canvas.clone());
        }

        canvas.restore();
    }
}
