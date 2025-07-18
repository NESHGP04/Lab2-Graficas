use raylib::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{thread, time};
use rand::Rng;

// Tamaño de la grilla
const GRID_SIZE: i32 = 100;
const CELL_SIZE: i32 = 5; // Tamaño visual de cada celda

fn main() {
    let (screen_width, screen_height) = (GRID_SIZE * CELL_SIZE, GRID_SIZE * CELL_SIZE);

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life - Raylib")
        .build();

    rl.set_target_fps(10); // 10 generaciones por segundo

    let mut grid = seed_pattern();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        draw_grid(&mut d, &grid);

        grid = next_generation(&grid);

        let total_population: i32 = grid.values().sum();
        println!("Population: {total_population}");
    }
}

// Avanza una generación según las reglas de Conway
fn next_generation(grid: &HashMap<(i32, i32), i32>) -> HashMap<(i32, i32), i32> {
    let mut new_grid: HashMap<(i32, i32), i32> = Default::default();

    let mut to_check: HashSet<(i32, i32)> = HashSet::new();

    for &pos in grid.keys() {
        to_check.insert(pos);
        for nb in neighbour_coords(pos.0, pos.1) {
            to_check.insert(nb);
        }
    }

    for pos in to_check {
        let val = *grid.get(&pos).unwrap_or(&0);
        let neighbors = neighbour_coords(pos.0, pos.1);
        let mut population = 0;

        for nb in neighbors {
            population += grid.get(&nb).unwrap_or(&0);
        }

        if pos.0 < 0 || pos.0 >= GRID_SIZE || pos.1 < 0 || pos.1 >= GRID_SIZE {
            continue; // Ignora fuera de límites
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

// Dibuja la grilla usando Raylib
fn draw_grid(d: &mut RaylibDrawHandle, grid: &HashMap<(i32, i32), i32>) {
    for (&(x, y), &value) in grid.iter() {
        if value == 1 {
            d.draw_rectangle(
                x * CELL_SIZE,
                y * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                Color::new(148, 212, 195, 255),
            );
        }
    }
}

fn place_glider(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.insert((x, y + 1), 1);
    grid.insert((x + 1, y + 2), 1);
    grid.insert((x + 2, y), 1);
    grid.insert((x + 2, y + 1), 1);
    grid.insert((x + 2, y + 2), 1);
}

fn place_block(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.insert((x, y), 1);
    grid.insert((x + 1, y), 1);
    grid.insert((x, y + 1), 1);
    grid.insert((x + 1, y + 1), 1);
}

fn place_spaceship(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
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

// Inicializa la grilla con celdas aleatorias
fn seed_pattern() -> HashMap<(i32, i32), i32> {
    let mut grid = HashMap::new();

    // Colocar gliders en posiciones aleatorias
    for i in 0..20 {
        let x = (i * 8) % GRID_SIZE;  // Distribución horizontal
        let y = (i * 5) % GRID_SIZE;  // Distribución vertical
        place_glider(&mut grid, x, y);
    }

    // Colocar bloques en posiciones fijas
    for i in 0..10 {
        let x = (i * 10) % GRID_SIZE;
        let y = ((i * 7) + 15) % GRID_SIZE;
        place_block(&mut grid, x, y);
    }

    // Colocar spaceships en distintas posiciones
    for i in 0..10 {
        let x = ((i * 12) + 30) % GRID_SIZE;
        let y = ((i * 9) + 40) % GRID_SIZE;
        place_spaceship(&mut grid, x, y);
    }

    grid
}

// Devuelve las coordenadas vecinas de una celda
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
