mod colors;
mod game_manager;
mod game_states;
mod interval;
mod overlays;
mod sprite_manager;
mod sprites;
mod utils;
mod vector;

extern crate js_sys;
extern crate web_sys;

use game_manager::GameManager;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use crate::sprites::sprite::CanvasDimension;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    game_manager: GameManager,
    canvas_context: CanvasRenderingContext2d,
}

/// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();

        let document = window()
            .expect("> No 'window' found in html page!")
            .document()
            .expect("> No 'document' found in the html page!");

        let option_element = document.get_element_by_id("canvas");

        if let Some(element) = option_element {
            let html_canvas_element: HtmlCanvasElement = element
                .dyn_into()
                .expect("> No 'canvas' found in html page!");

            let canvas_dimension = CanvasDimension {
                width: html_canvas_element.width() as f64,
                height: html_canvas_element.height() as f64,
            };

            let context_object = html_canvas_element.get_context("2d").unwrap().unwrap();

            let canvas_context = context_object
                .dyn_into()
                .expect("> Error when trying to get the canvas element in the html document!");

            Game {
                game_manager: GameManager::new(canvas_dimension),
                canvas_context,
            }
        } else {
            panic!("> No canvas element found in the html document!");
        }
    }

    pub fn tick(&mut self) {
        self.game_manager.update();
        self.game_manager.draw(self.canvas_context.clone());
    }

    pub fn key_pressed(&mut self, key: &str) {
        self.game_manager.key_pressed(key);
    }

    pub fn key_released(&mut self, key: &str) {
        self.game_manager.key_released(key);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
