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
use crate::sprites::potatoid::Potatoid;
use crate::sprites::sprite::{Spritable, Sprite};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static DEAD_COLOR: &str = "#242c37";
static ALIVE_COLOR: &str = "#4fe4c1";

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Game {
    canvas_width: f64,
    canvas_height: f64,
    canvas: CanvasRenderingContext2d,
    sprite: Sprite<BasicSprite>,
    sprites: Vec<Sprite<Potatoid>>,
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

        // When testing, there's no index.html page containing the canvas element with canvas as id
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

            let sprite = Sprite::new(BasicSprite::new(), canvas_width, canvas_height);

            let mut sprites = Vec::new();

            for _count in 0..20 {
                sprites.push(Sprite::new(Potatoid::new(), canvas_width, canvas_height))
            }

            Game {
                canvas_width,
                canvas_height,
                canvas,
                sprite,
                sprites,
            }
        } else {
            panic!("> No canvas element found in the html document!");
        }
    }

    pub fn tick(&mut self) {
        self.canvas
            .clear_rect(0f64, 0f64, self.canvas_width, self.canvas_height);

        self.test_canvas();

        self.sprite.update();
        self.sprite.draw(self.canvas.clone());

        for count in 0..20 {
            self.sprites[count].update();
            self.sprites[count].draw(self.canvas.clone());
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    fn test_canvas(&mut self) {
        self.canvas.begin_path();
        self.canvas.set_fill_style(&DEAD_COLOR.into());
        self.canvas.fill_rect(100f64, 100f64, 100f64, 100f64);
        self.canvas.close_path();
    }
}
