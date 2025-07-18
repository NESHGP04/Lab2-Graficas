use raylib::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::GRID_SIZE;

pub fn next_generation(grid: &HashMap<(i32, i32), i32>) -> HashMap<(i32, i32), i32> {
    let mut new_grid: HashMap<(i32, i32), i32> = Default::default();
    let mut to_check: HashSet<(i32, i32)> = HashSet::new();

    for &pos in grid.keys() {
        to_check.insert(pos);
        for nb in neighbour_coords(pos.0, pos.1) {
            to_check.insert(nb);
        }
    }

    for pos in to_check {
        if pos.0 < 0 || pos.0 >= GRID_SIZE || pos.1 < 0 || pos.1 >= GRID_SIZE {
            continue;
        }

        let val = *grid.get(&pos).unwrap_or(&0);
        let mut population = 0;
        for nb in neighbour_coords(pos.0, pos.1) {
            population += grid.get(&nb).unwrap_or(&0);
        }

        if val == 1 && (population == 2 || population == 3) {
            new_grid.insert(pos, 1);
        } else if val == 0 && population == 3 {
            new_grid.insert(pos, 1);
        } else {
            new_grid.insert(pos, 0);
        }
    }

    new_grid
}

pub fn draw_grid(d: &mut RaylibDrawHandle, grid: &HashMap<(i32, i32), i32>) {
    for (&(x, y), &value) in grid.iter() {
        if value == 1 {
            d.draw_rectangle(
                x * super::CELL_SIZE,
                y * super::CELL_SIZE,
                super::CELL_SIZE,
                super::CELL_SIZE,
                Color::new(148, 212, 195, 255), // Color celda viva
            );
        }
    }
}

fn neighbour_coords(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x + 1, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ]
}
