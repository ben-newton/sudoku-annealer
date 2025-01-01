// Module declaration
mod modules {
    pub mod grid;
    pub mod reader;
    pub mod scorer;
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
    let  current_grid = match SudokuGrid::new(binary, initial_grid) {
        Ok(g) => g,
        Err(err) => {
            println!("Error reading grid: {}", err);
            process::exit(1);
        }
    };

    for _ in 1..100 {

    }
    let temperature: u8 = 0;
    SudokuGrid::perform_move(&current_grid, temperature);

    // let mut delta: f64 = 0.1;
    // let mut alpha:u8 = 0;
    // let mut n: u8 = 0;
}