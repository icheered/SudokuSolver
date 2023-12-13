#[allow(unused_variables, unused_imports)]
use crate::reduced_options;

use crate::types::{Grid, GRID_SIZE};

pub fn solve(sudoku: &Grid) -> Option<Grid>{
    // Create a mutable copy of the input grid
    let mut sudoku_copy = *sudoku; 

    // Initialize the bitsets for rows, columns, and squares
    let mut row_sets = [0u16; GRID_SIZE];
    let mut col_sets = [0u16; GRID_SIZE];
    let mut square_sets = [[0u16; 3]; 3];

    // Populate the bitsets based on the initial state of the Sudoku grid
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if sudoku_copy[i][j] != 0 {
                let n = sudoku_copy[i][j] as usize;
                row_sets[i] |= 1 << n;
                col_sets[j] |= 1 << n;
                square_sets[i / 3][j / 3] |= 1 << n;
            }
        }
    }
    reduced_options::backtrack(&mut sudoku_copy, &mut row_sets, &mut col_sets, &mut square_sets)
}
