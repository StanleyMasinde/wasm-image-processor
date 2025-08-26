use wasm_bindgen::prelude::wasm_bindgen;

pub mod color_filters;
pub mod transformation;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(m: &str);
}
