mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

const WASM_MEMORY_BUFFER_SIZE: usize = 2;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

const WORLD_WIDTH: usize = 256;
const WORLD_HEIGHT: usize = 256;
const PIXEL_SIZE_IN_BYTES: usize = 4; // RGBA

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    rendered_image: [u8; WORLD_WIDTH * WORLD_HEIGHT * PIXEL_SIZE_IN_BYTES],
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        // TODO: move it somewhere
        utils::set_panic_hook();

        let cells = (0..WORLD_WIDTH * WORLD_HEIGHT)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        World {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            cells,
            rendered_image: [0xff; WORLD_WIDTH * WORLD_HEIGHT * PIXEL_SIZE_IN_BYTES],
        }
    }

    // TODO: We should return size of rendered image not cells
    // TODO: Ok, another way - do we need grid lines?
    pub fn width(&self) -> u32 {
        self.width as u32
    }

    // TODO: We should return size of rendered image not cells
    // TODO: Ok, another way - do we need grid lines?
    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn rendered_image_ptr(&self) -> *const u8 {
        self.rendered_image.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut cells_next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = Self::cell_index(col, row, self.width);
                let cell = self.cells[idx];
                let alive_neighbours = self.alive_neighbours_count(col, row);
                let new_state = match (cell, alive_neighbours) {
                    // Any live cell with less than two neighbours dies out of underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Any live cell with two or three neighbours lives to next generation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Any live cell with more than three neighbours dies out of overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Any dead cell with three neighbours revives
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                cells_next[idx] = new_state;
            }
        }
        self.cells = cells_next;
    }

    fn cell_index(column: usize, row: usize, width: usize) -> usize {
        width * row + column
    }

    fn alive_neighbours_count(&self, column: usize, row: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = Self::cell_index(neighbor_col, neighbor_row, self.width);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn render(&mut self) {
        let mut idx: usize = 0;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let color = if cell == Cell::Dead { 0xff } else { 0x00 };
                self.rendered_image[idx] = color; // R
                self.rendered_image[idx + 1] = color; // G
                self.rendered_image[idx + 2] = color; // B
                self.rendered_image[idx + 3] = 255; // A
                idx += 4;
            }
        }
    }
}
