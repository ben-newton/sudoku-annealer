// use std::error::Error;
// use super::grid::SudokuGrid;
// pub struct SudokuScorer;

// impl SudokuScorer {
//     pub fn count_duplicates(numbers: &[u8]) -> Vec<(u8, u32)> {
//         let mut counts = vec![0; 10];
//         for &num in numbers {
//             counts[num as usize] += 1;
//         }
        
//         counts.iter()
//             .enumerate()
//             .filter(|(_, &count)| count > 1)
//             .map(|(num, &count)| (num as u8, count))
//             .collect()
//     }

//     pub fn score(sudoku_grid: &SudokuGrid) -> Result<u32, Box<dyn Error>> {
//         // Check rows
//         let mut row_score = 0;
//         for row_idx in 0..9 {
//             let row = sudoku_grid.get_row(row_idx);
//             let duplicates = Self::count_duplicates(row);
//             row_score += duplicates.iter()
//                 .map(|(_, count)| count - 1)
//                 .sum::<u32>();
//         }
    
//         // Check columns
//         let mut col_score: u32 = 0;
//         for col_idx in 0..9 {
//             let column = sudoku_grid.get_column(col_idx);
//             let duplicates = Self::count_duplicates(&column);
//             col_score += duplicates.iter()
//                 .map(|(_, count)| count - 1)
//                 .sum::<u32>();
//         }
        
//         let total_score: u32 = row_score + col_score;
//         Ok(total_score)
//     }

// } 