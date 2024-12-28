use std::{error::Error, fs::File, ffi::OsString};

pub fn read_sudoku(file_path: OsString) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    // ... rest of reading logic ...
} 