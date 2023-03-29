use wasm_bindgen::prelude::wasm_bindgen;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn map(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

#[wasm_bindgen]
extern "C" {
    pub fn millis() -> f64;
}
