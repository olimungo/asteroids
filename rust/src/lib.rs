mod overlays;
mod sprites;
mod utils;
mod vector;

extern crate js_sys;
extern crate web_sys;

use overlays::hub::Hub;
use web_sys::window;
// use web_sys::console;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use crate::sprites::potatoid::Potatoid;
use crate::sprites::sprite::Canvas;
use crate::sprites::sprite::Spritable;
use crate::vector::Vector;
use sprites::sprite::SpriteData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
pub struct Game {
    canvas_width: f64,
    canvas_height: f64,
    canvas: CanvasRenderingContext2d,
    sprites: Vec<Potatoid>,
    overlay_hub: Hub,
}

/// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();

        let canvas: CanvasRenderingContext2d;

        let document = window()
            .expect("> No 'window' found in html page!")
            .document()
            .expect("> No 'document' found in the html page!");

        let option_element = document.get_element_by_id("canvas");

        if let Some(element) = option_element {
            let html_canvas_element: HtmlCanvasElement = element
                .dyn_into()
                .expect("> No 'canvas' found in html page!");
            let canvas_width = html_canvas_element.width() as f64;
            let canvas_height = html_canvas_element.height() as f64;
            let context_object = html_canvas_element.get_context("2d").unwrap().unwrap();

            canvas = context_object
                .dyn_into()
                .expect("> Error when trying to get the canvas element in the html document!");

            let mut sprites = Vec::new();

            let sprites_count = 20;

            for _count in 0..sprites_count {
                let mut position = Vector::random();
                position = position + Vector::new(canvas_width / 2.0, canvas_height / 2.0);

                let velocity = Vector::random();
                let diameter = rand::thread_rng().gen_range(50.0..100.0);
                let rotation = 0.0;
                let rotation_step = rand::thread_rng().gen_range(-0.05..0.05);

                sprites.push(Potatoid::new(
                    SpriteData {
                        position,
                        velocity,
                        diameter,
                        rotation,
                        rotation_step,
                    },
                    8u8,
                    Canvas {
                        width: canvas_width,
                        height: canvas_height,
                    },
                ));
            }

            let overlay_hub = Hub::new(canvas_width, canvas_height);

            Game {
                canvas_width,
                canvas_height,
                canvas,
                sprites,
                overlay_hub,
            }
        } else {
            panic!("> No canvas element found in the html document!");
        }
    }

    pub fn tick(&mut self) {
        self.canvas
            .clear_rect(0f64, 0f64, self.canvas_width, self.canvas_height);

        for count in 0..self.sprites.len() {
            self.sprites[count].update();
            self.sprites[count].draw(self.canvas.clone());
        }

        self.overlay_hub.draw(self.canvas.clone());
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
