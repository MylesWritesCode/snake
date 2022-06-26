use wasm_bindgen::prelude::*;

mod snake;
use snake::SnakeGame;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting web server".into());
}