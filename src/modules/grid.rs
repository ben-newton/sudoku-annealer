pub struct SudokuGrid {
    grid: Vec<Vec<u8>>
}

impl SudokuGrid {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        Self { grid }
    }
    pub fn get_row(&self, row: usize) -> &[u8] {
        &self.grid[row]
    }

    pub fn get_column(&self, col: usize) -> Vec<u8> {
        self.grid.iter().map(|row| row[col]).collect()
    }
}
