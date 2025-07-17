use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};
use raylib::prelude::*;

//MAIN
fn main() {
    let print_area = 60; // grid 60x60
    let mut grid = seed_grid(print_area);

    // Setup Raylib
    let cell_size = 10;
    let (screen_width, screen_height) = (print_area * cell_size, print_area * cell_size);
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life - Raylib")
        .build();

    rl.set_target_fps(10);

    let mut generation = 0;

    //Main Loop
    while !rl.window_should_close() {
        let mut new_grid: HashMap<(i32, i32), i32> = Default::default();
        let keys: Vec<&(i32, i32)> = grid.keys().collect();

        for key in keys {
            let val = grid[key];
            let (x, y) = key;
            let mut population = 0;
            let nb_coords = neighbour_coords(*x, *y);
            for coord in &nb_coords {
                if grid.contains_key(coord) {
                    population += grid[coord];
                }
            }

            if val == 1 && (population == 2 || population == 3) {
                new_grid.insert(key.clone(), 1);
                for coord in &nb_coords {
                    if !new_grid.contains_key(coord) {
                        new_grid.insert(coord.clone(), 0);
                    }
                }
            }

            if val == 0 && population == 3 {
                new_grid.insert(key.clone(), 1);
                for coord in &nb_coords {
                    if !new_grid.contains_key(coord) {
                        new_grid.insert(coord.clone(), 0);
                    }
                }
            }
        }

        grid = new_grid;
        generation += 1;
        let total_population: i32 = grid.values().sum();

        // Dibuja usando Raylib
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_grid(&mut d, print_area, cell_size, &grid);

        d.draw_text(
            &format!("Generation: {generation} | Population: {total_population}"),
            10,
            10,
            20,
            Color::BLACK,
        );

        thread::sleep(time::Duration::from_millis(100)); // Mantiene el ritmo
    }
}

// Dibuja la grilla en Raylib
fn draw_grid(d: &mut RaylibDrawHandle, print_area: i32, cell_size: i32, grid: &HashMap<(i32, i32), i32>) {
    for y in 0..print_area {
        for x in 0..print_area {
            let key = (x, y);
            if grid.get(&key).unwrap_or(&0) == &1 {
                d.draw_rectangle(
                    x * cell_size,
                    y * cell_size,
                    cell_size,
                    cell_size,
                    Color::new(148, 212, 195, 255),
                );
            }
        }
    }
}

// Funciones auxiliares
fn seed_grid(print_area: i32) -> HashMap<(i32, i32), i32> {
    let mut grid: HashMap<(i32, i32), i32> = Default::default();
    for _ in 0..35 {
        let x = rand::thread_rng().gen_range(print_area / 4..print_area - (print_area / 4));
        let y = rand::thread_rng().gen_range(print_area / 4..print_area - (print_area / 4));
        grid.insert((x, y), 1);
        grid.insert((x, y + 1), 1);
        grid.insert((x + 1, y), 1);

        for coord in neighbour_coords(x, y) {
            grid.entry(coord).or_insert(0);
        }
        for coord in neighbour_coords(x, y + 1) {
            grid.entry(coord).or_insert(0);
        }
        for coord in neighbour_coords(x + 1, y) {
            grid.entry(coord).or_insert(0);
        }
    }
    grid
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
