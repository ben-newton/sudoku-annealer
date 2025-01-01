use std::{error::Error, fs::File, ffi::OsString};

pub fn read_sudoku(file_path: OsString) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
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