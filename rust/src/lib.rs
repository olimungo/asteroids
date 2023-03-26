mod sprites;
mod utils;
mod vector;

extern crate fixedbitset;
extern crate js_sys;
extern crate web_sys;

use web_sys::window;
// use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use crate::sprites::basic_sprite::BasicSprite;
use crate::sprites::sprite::{Spritable, Sprite};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static DEAD_COLOR: &str = "#242c37";
static ALIVE_COLOR: &str = "#4fe4c1";

#[wasm_bindgen]
pub struct Game {
    canvas: Option<CanvasRenderingContext2d>,
    sprite: Sprite<BasicSprite>,
}

/// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();

        let mut canvas: Option<CanvasRenderingContext2d> = None;
        let document = window().unwrap().document().unwrap();
        let option_element = document.get_element_by_id("canvas");

        // When testing, there's no index.html page containing the canvas element with ui-canvas as id
        if let Some(element) = option_element {
            let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
            let context_object = html_canvas_element.get_context("2d").unwrap().unwrap();
            canvas = Some(context_object.dyn_into().unwrap());
        }

        let sprite = Sprite::new(BasicSprite::new(), canvas.clone());

        Game { canvas, sprite }
    }

    pub fn tick(&mut self) {
        if let Some(canvas) = &self.canvas {
            canvas.clear_rect(0f64, 0f64, 500f64, 500f64);
        }

        self.test_canvas();

        self.sprite.update();
        self.sprite.draw();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    fn test_canvas(&mut self) {
        if let Some(canvas) = &self.canvas {
            canvas.begin_path();
            canvas.set_fill_style(&DEAD_COLOR.into());
            canvas.fill_rect(0f64, 0f64, 20f64, 20f64);
            canvas.close_path();
        }
    }
}
