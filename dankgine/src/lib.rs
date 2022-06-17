use wasm_bindgen::prelude::*;

pub mod bodies;
pub mod engine;
pub mod world;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
