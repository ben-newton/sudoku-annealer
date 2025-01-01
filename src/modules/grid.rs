use super::reader;
use std::{ffi::OsString, process};
use num_traits::pow;
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;


pub struct SudokuGrid {
    grid: Vec<Vec<u8>>,
    binary: Vec<Vec<u8>>
}
pub struct InitialiseGrid{
    grid: Vec<Vec<u8>>
}

impl SudokuGrid {
    pub fn new(binary: Vec<Vec<u8>>, grid: Vec<Vec<u8>>) -> Result<Self, &'static str> {
        // Validate grid dimensions
        if binary.len() != 9 {
            return Err("Grid must have exactly 9 rows");
        }
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
        
        Ok(Self { grid, binary })
    }
    pub fn get_row(&self, grid: &Vec<Vec<u8>>, row: usize) -> Vec<u8> {
        grid[row].clone()
    }

    pub fn get_column(&self, grid: &Vec<Vec<u8>>, col: usize) -> Vec<u8> {
        grid.iter().map(|row| row[col]).collect()
    }

    fn blocks_to_grid(&self, blocks: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut grid: Vec<Vec<u8>> = vec![vec![1;9];9];
        for (block_index, block) in blocks.iter().enumerate() {
            let block_row = block_index / 3;  // Calculate block row position
            let block_col = block_index % 3;

            for i in 0..3 {
                for j in 0..3 {
                    let row = block_row * 3 + i;
                    let col = block_col * 3 + j;
                    grid[row][col] = block[i * 3 + j];
                }
            }
        }
        grid
    }
    fn get_blocks(&self, grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{ 
        let mut block_vectors: Vec<Vec<u8>> = vec![];
        for block_row in 0..3 {
            for block_col in 0..3 {
                block_vectors.push(self.get_block(grid, block_row, block_col));
            }
        }
        block_vectors
    }

    fn get_block(&self, grid: &Vec<Vec<u8>>, block_row: usize, block_col: usize) -> Vec<u8> {
        let mut block_vector = vec![];  // Initialize empty vector
        for i in 0..3 {
            for j in 0..3 {
                let row = block_row * 3 + i;
                let col = block_col * 3 + j;
                block_vector.push(grid[row][col]);
            }
        }
        block_vector
    }
    pub fn count_duplicates(numbers: &[u8]) -> Vec<(u8, u32)> {
        let mut counts = vec![0; 10];
        for &num in numbers {
            counts[num as usize] += 1;
        }
        
        counts.iter()
            .enumerate()
            .filter(|(_, &count)| count > 1)
            .map(|(num, &count)| (num as u8, count))
            .collect()
    }
    pub fn score(&self, grid: &Vec<Vec<u8>>) -> u32 {
        // Check rows
        let mut row_score = 0;
        for row_idx in 0..9 {
            let row = self.get_row(&grid, row_idx);
            let duplicates = Self::count_duplicates(&row);
            row_score += duplicates.iter()
                .map(|(_, count)| count - 1)
                .sum::<u32>();
        }
    
        // Check columns
        let mut col_score: u32 = 0;
        for col_idx in 0..9 {
            let column = self.get_column(&grid, col_idx);
            let duplicates = Self::count_duplicates(&column);
            col_score += duplicates.iter()
                .map(|(_, count)| count - 1)
                .sum::<u32>();
        }
        
        let total_score: u32 = row_score + col_score;
        total_score
    }

    fn calculate_move(&self, grid: &Vec<Vec<u8>>) -> u32 {
        // match self.score(grid) {
        //     Ok(score) => println!("Total score: {}", score),
        //     Err(err) => {
        //         println!("Error scoring grid: {}", err);
        //         process::exit(1);
        //     }
        // }
        self.score(grid)
    }

    pub fn perform_move(&self, temperature: u8) -> Vec<Vec<u8>>{        
        let mut rng = rand::thread_rng();
        let blocks: Vec<Vec<u8>> = self.get_blocks(&self.grid);
        let binary_blocks: Vec<Vec<u8>> = self.get_blocks(&self.binary);

        let block_index = (0..blocks.len()).choose(&mut rng).unwrap();
        let chosen_block = &blocks[block_index];
        let chosen_binary_block = &binary_blocks[block_index];
        println!("Chosen block: {:?}", chosen_block);
        println!("Chosen binary block: {:?}", chosen_binary_block);
        
        let available_indices: Vec<usize> = chosen_binary_block.iter()
            .enumerate()
            .filter(|&(_, &val)| val == 0)
            .map(|(index, _)| index)
            .collect();

        if available_indices.len() >= 2 {
            let selected_indices: Vec<usize> = available_indices
                .choose_multiple(&mut rng, 2)
                .cloned()
                .collect();
            println!("Swapping values at positions {} and {}", 
                selected_indices[0], selected_indices[1]);
            println!("Values to swap: {} and {}", 
                chosen_block[selected_indices[0]], chosen_block[selected_indices[1]]);
            // Create a new block with the swapped values
            let mut new_block = chosen_block.clone();
            new_block.swap(selected_indices[0], selected_indices[1]);

            let mut new_blocks = blocks.clone();
            new_blocks[block_index] = new_block;
            let new_grid = self.blocks_to_grid(new_blocks);
            println!("{:?}", self.grid);
            println!("{:?}", new_grid);
            let old_score = self.calculate_move(&self.grid);
            let new_score = self.calculate_move(&new_grid);
            println!("old score: {:?}", old_score);
            println!("new score: {:?}", new_score);

            if new_score < old_score {
                new_grid
            }
            else {
                let energy_difference = new_score - old_score;
                let exp: f64 = -(energy_difference as f64) / (temperature as f64);
                let probability = std::f64::consts::E.powf(exp);
                let random_value: f64 = rand::random();
                if random_value <= probability {
                    new_grid
                } else {
                    self.blocks_to_grid(blocks)
                }
            }
            }
        else {
            self.blocks_to_grid(blocks)
        }
    }
}

impl InitialiseGrid {
    pub fn new(file_path: OsString) -> Self {
        let filegrid = match reader::read_sudoku(file_path) {
            Ok(g) => g,
            Err(err) => {
                println!("Error reading grid: {}", err);
                process::exit(1);
            }
        };
        // Store the original grid format instead of blocks
        Self { grid: filegrid }
    }

    pub fn generate_initial(&self) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        let mut initial_grid = self.grid.clone();
        
        let binary_grid: Vec<Vec<u8>> = self.grid.iter()
            .map(|row| row.iter()
                .map(|&val| if val != 0 { 1 } else { 0 })
                .collect())
            .collect();

        // Fill each 3x3 block
        for block_row in 0..3 {
            for block_col in 0..3 {
                let filled_block =  self.fill_block(&mut initial_grid, block_row, block_col);
                for i in 0..3 {
                    for j in 0..3 {
                        let row = block_row * 3 + i;
                        let col = block_col * 3 + j;
                        initial_grid[row][col] = filled_block[i * 3 + j];
                    }
                }
            }
        }
        (binary_grid, initial_grid)
    }

    pub fn get_blocks(&self, grid: &Vec<Vec<u8>>, block_row: usize, block_col: usize) -> Vec<u8> {
        let mut block_vector = vec![];  // Initialize empty vector
        for i in 0..3 {
            for j in 0..3 {
                let row = block_row * 3 + i;
                let col = block_col * 3 + j;
                block_vector.push(grid[row][col]);
            }
        }
        block_vector
    }

    fn fill_block(&self, grid: &mut Vec<Vec<u8>>, block_row: usize, block_col: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let block_vector = self.get_blocks(&grid, block_row, block_col);
        // Get available numbers for this cell
        let mut available: Vec<u8> = (1..=9).collect();
        available.retain(|&num| !block_vector.contains(&num));
        available.shuffle(&mut rng);

        let mut value_server = available.iter();

        let mut filled_block = vec![];
        for val in block_vector.iter(){
            if *val == 0 {
                filled_block.push(*value_server.next().unwrap());
            }
            else {
                filled_block.push(*val);
            }
        }
        filled_block
    }
}