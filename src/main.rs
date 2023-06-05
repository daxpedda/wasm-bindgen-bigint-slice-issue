use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn test(_: &[u64]) {}

fn main() {
    console_error_panic_hook::set_once();
}
