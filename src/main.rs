// Module declaration
mod modules {
    pub mod grid;
    pub mod reader;
    pub mod scorer;
}

// Local module imports
use modules::{
    grid::SudokuGrid,
    scorer::SudokuScorer,
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

    let grid = match SudokuGrid::new(file_path) {
        Ok(g) => g,
        Err(err) => {
            println!("Error reading grid: {}", err);
            process::exit(1);
        }
    };

    let initial_grid = SudokuGrid::initial_grid(&grid);

    match SudokuScorer::score(&grid) {
        Ok(score) => println!("Total score: {}", score),
        Err(err) => {
            println!("Error scoring grid: {}", err);
            process::exit(1);
        }
    }
}