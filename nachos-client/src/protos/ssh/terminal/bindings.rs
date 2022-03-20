use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Terminal;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Terminal;

    #[wasm_bindgen(method)]
    pub fn open(this: &Terminal, parent: web_sys::Element);

    #[wasm_bindgen(method)]
    pub fn write(this: &Terminal, data: Vec<u8>);
}
