#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/nachos.js")]
extern "C" {
    pub type NachosClient;

    #[wasm_bindgen(constructor)]
    pub fn new(tunnelUrl: &str) -> NachosClient;

    #[wasm_bindgen(method)]
    pub fn connect(this: &NachosClient);

    #[wasm_bindgen(method)]
    pub fn isConnected(this: &NachosClient) -> bool;
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
