mod utils;

use wasm_bindgen::prelude::*;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum CellState {
    Dead,
    Alive,
}

#[wasm_bindgen]
pub fn init_world() {
    // TODO: receive seed as an argument
    // TODO: init world here
}