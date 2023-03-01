use wasm_bindgen::prelude::*;

pub mod data;

#[wasm_bindgen]
pub fn foo() -> String {
    "hello".to_string()
}
