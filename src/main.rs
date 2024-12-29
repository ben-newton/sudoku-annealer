// Module declaration
mod modules {
    pub mod grid;
    pub mod reader;
    pub mod scorer;
}

// Local module imports
use modules::{
    grid::SudokuGrid,
    reader::read_sudoku,
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

    let grid = match read_sudoku(file_path) {
        Ok(g) => SudokuGrid::new(g),
        Err(err) => {
            println!("Error reading grid: {}", err);
            process::exit(1);
        }
    };

    match SudokuScorer::score(&grid) {
        Ok(score) => println!("Total score: {}", score),
        Err(err) => {
            println!("Error scoring grid: {}", err);
            process::exit(1);
        }
    }
}