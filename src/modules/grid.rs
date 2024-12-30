use super::reader;
use std::{ffi::OsString, process};

pub struct SudokuGrid {
    grid: Vec<Vec<u8>>
}

impl SudokuGrid {
    pub fn new(file_path: OsString) -> Result<Self, &'static str> {
        let grid = match reader::read_sudoku(file_path) {
            Ok(g) => g,
            Err(err) => {
                println!("Error reading grid: {}", err);
                process::exit(1);
            }
        };
        // Validate grid dimensions
        if grid.len() != 9 {
            return Err("Grid must have exactly 9 rows");
        }
        
        for row in &grid {
            if row.len() != 9 {
                return Err("Each row must have exactly 9 columns");
            }
            
            // Validate numbers are in range 0-9
            if !row.iter().all(|&num| num <= 9) {
                return Err("All numbers must be between 0 and 9");
            }
        }
        
        Ok(Self { grid })
    }
    pub fn get_row(&self, row: usize) -> &[u8] {
        &self.grid[row]
    }

    pub fn get_column(&self, col: usize) -> Vec<u8> {
        self.grid.iter().map(|row| row[col]).collect()
    }
    pub fn get_blocks(&self) -> Vec<Vec<u8>> {
            let mut blocks =vec![];
            for block_col in 0..3 {
                for block_row in 0..3 {
                    let mut block_vector = vec![];
                    for i in 0..3 {
                        for j in 0..3 {
                            let row = block_row * 3 + i;
                            let col = block_col * 3 + j;
                            block_vector.push(self.grid[row][col]);
                        }
                    }
                    blocks.push(block_vector);
                }
            }
            blocks
        }
    pub fn initial_grid(&self) -> Vec<Vec<u8>> {
        self.get_blocks()
    }
}
