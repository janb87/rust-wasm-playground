extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../domUtils")]
extern {
    fn appendStringToBody(x: &str);
    fn alert(message: &str);
}

#[wasm_bindgen]
pub fn run() {
    appendStringToBody("Hello World");
    alert("Hello World");
}