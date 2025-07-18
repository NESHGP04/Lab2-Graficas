use std::collections::HashMap;

use crate::GRID_SIZE;

pub fn seed_pattern() -> HashMap<(i32, i32), i32> {
    let mut grid = HashMap::new();

    for i in 0..10 {
        let x = (i * 8) % GRID_SIZE;
        let y = (i * 5) % GRID_SIZE;
        place_glider(&mut grid, x, y);
    }

    for i in 0..8 {
        let x = (i * 10) % GRID_SIZE;
        let y = ((i * 7) + 15) % GRID_SIZE;
        place_block(&mut grid, x, y);
    }

    for i in 0..5 {
        let x = ((i * 12) + 30) % GRID_SIZE;
        let y = ((i * 9) + 40) % GRID_SIZE;
        place_spaceship(&mut grid, x, y);
    }

    grid
}

pub fn place_glider(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.insert((x, y + 1), 1);
    grid.insert((x + 1, y + 2), 1);
    grid.insert((x + 2, y), 1);
    grid.insert((x + 2, y + 1), 1);
    grid.insert((x + 2, y + 2), 1);
}

pub fn place_block(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.insert((x, y), 1);
    grid.insert((x + 1, y), 1);
    grid.insert((x, y + 1), 1);
    grid.insert((x + 1, y + 1), 1);
}

pub fn place_spaceship(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.insert((x + 1, y), 1);
    grid.insert((x + 4, y), 1);
    grid.insert((x, y + 1), 1);
    grid.insert((x, y + 2), 1);
    grid.insert((x + 4, y + 1), 1);
    grid.insert((x + 4, y + 2), 1);
    grid.insert((x + 1, y + 3), 1);
    grid.insert((x + 2, y + 3), 1);
    grid.insert((x + 3, y + 3), 1);
    grid.insert((x + 4, y + 3), 1);
}
