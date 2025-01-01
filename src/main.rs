// Module declaration
mod modules {
    pub mod grid;
    pub mod reader;
}

// Local module imports
use modules::{
    grid::SudokuGrid,
    grid::InitialiseGrid
};

use std::{env, process};

fn main() {
    let file_path = match env::args_os().nth(1) {
        Some(path) => path,
        None => {
            println!("Expected 1 argument, but got none");
            process::exit(1);
        }
    };
    let grid_initialiser = InitialiseGrid::new(file_path);
    let (binary, initial_grid) = grid_initialiser.generate_initial();
    let mut current_grid = match SudokuGrid::new(binary, initial_grid) {
        Ok(g) => g,
        Err(err) => {
            println!("Error reading grid: {}", err);
            process::exit(1);
        }
    };
    let n = 1500000;
    let mut temperature: f64 = 1.;
    let delta: f64 = 2.5;
    let alpha: f64 = 0.98;
    let mut iteration_count: u16 = 0;
    let max_iter: u16 = 200;
    let mut best_score: u32 = 10000;
    let mut best_grid: Vec<Vec<u8>> = vec![vec![]];
    let mut optimal = false;
    use std::time::Instant;
    let now = Instant::now();
    for i in 1..n {
        if iteration_count >= max_iter {
            // println!("temp: {}", temperature);
            temperature += delta;
            // println!("New temperature set: {:#?}", temperature);
            // println!("");
        }
        temperature = temperature * alpha;
        if SudokuGrid::perform_move(&mut current_grid, temperature){
            let new_score = SudokuGrid::current_score(&current_grid);
            if new_score < best_score {
                best_score = new_score;
                best_grid = SudokuGrid::current_grid(&current_grid);
            }
            // println!("New grid picked with score: {}", new_score);
            iteration_count = 0;
        }
        else {
            iteration_count += 1;
        }
        if best_score == 0 {
            println!("");
            println!("Optimal solution found: {:?}", best_grid);
            println!("Solution found in {} moves", i);
            optimal = true;
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            println!("");
            break
        }
    }
    if optimal!=true {
        println!("Best score: {}", best_score);
        println!("Best solution found: {:?}", best_grid);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        println!("");
    }
}