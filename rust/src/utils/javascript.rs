use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn millis() -> f64;
}

#[wasm_bindgen]
extern "C" {
    pub fn fps() -> u8;
}

#[wasm_bindgen]
extern "C" {
    pub fn getCookie() -> u32;
}

#[wasm_bindgen]
extern "C" {
    pub fn setCookie(top_score: u32);
}
