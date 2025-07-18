mod grid;
mod patterns;

use raylib::prelude::*;
use grid::{next_generation, draw_grid};
use patterns::{seed_pattern};

use std::collections::HashMap;

const GRID_SIZE: i32 = 100;
const CELL_SIZE: i32 = 5;

fn main() {
    let (screen_width, screen_height) = (GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE);

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life - Raylib")
        .build();

    rl.set_target_fps(10);

    let mut grid = seed_pattern();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        draw_grid(&mut d, &grid);

        grid = next_generation(&grid);

        let total_population: i32 = grid.values().sum();
        println!("Population: {total_population}");
    }
}
