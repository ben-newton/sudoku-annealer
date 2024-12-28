use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
};

fn main() {
    // let temp: u8 = 20;
    // let alpha: f64 = 0.98;
    // let delta: f64 = 2.5;
    let grid = match read_sudoku() {
        Ok(grid) => grid,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let grid_score: u32;
    grid_score = match score(grid) {
        Ok(grid_score) => grid_score,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    println!("{}", grid_score)
}

fn read_sudoku() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut grid = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let numbers: Result<Vec<u8>, _> = record
            .iter()
            .map(|s| s.parse::<u8>())
            .collect();
        grid.push(numbers?);
    }
    Ok(grid)
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn score(grid: Vec<Vec<u8>>) -> Result<u32, Box<dyn Error>> {
    fn count_duplicates(numbers: &[u8]) -> Vec<(u8, u32)> {
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
    fn get_block_numbers(grid: &[Vec<u8>], block_row: usize, block_col: usize) -> Vec<u8> {
        let mut numbers = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                let row = block_row * 3 + i;
                let col = block_col * 3 + j;
                numbers.push(grid[row][col]);
            }
        }
        numbers
    }
    // Check rows
    let mut row_score = 0;
    for row in grid.iter() {
        let duplicates = count_duplicates(row);
        row_score += duplicates.iter()
            .map(|(_, count)| count - 1)  // For each duplicate, count how many extras there are
            .sum::<u32>();
    }

    // Check columns
    let mut col_score: u32 = 0;
    let num_cols = grid[0].len();
    for col in 0..num_cols {
        let column: Vec<u8> = grid.iter().map(|row| row[col]).collect();
        let duplicates = count_duplicates(&column);
        col_score += duplicates.iter()
            .map(|(_, count)| count - 1)
            .sum::<u32>();
    }

    // Check 3x3 blocks
    let mut block_score: u32 = 0;
    for block_row in 0..3 {
        for block_col in 0..3 {
            let block_numbers = get_block_numbers(&grid, block_row, block_col);
            let duplicates = count_duplicates(&block_numbers);
            if !duplicates.is_empty() {
                block_score += duplicates.iter()
                    .map(|(_, count)| count - 1)
                    .sum::<u32>();
            }
        }
    }
    let total_score: u32 = row_score+col_score+block_score;
    Ok(total_score)
}