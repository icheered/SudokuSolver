use crate::types::{Grid, BitSet, GRID_SIZE};
fn get_unavailable(row_sets: &[BitSet], col_sets: &[BitSet], square_sets: &[[BitSet; 3]; 3], i: usize, j: usize) -> BitSet {
    row_sets[i] | col_sets[j] | square_sets[i / 3][j / 3]
}

pub fn backtrack(sudoku: &mut Grid, row_sets: &mut [BitSet], col_sets: &mut [BitSet], square_sets: &mut [[BitSet; 3]; 3]) -> Option<Grid> {
    let mut best_cell = None;
    let mut fewest_options = GRID_SIZE + 1;

    // Find the cell with the fewest options
    'outer: for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if sudoku[i][j] == 0 {
                let unavailable = get_unavailable(row_sets, col_sets, square_sets, i, j);
                let options_count = GRID_SIZE - (unavailable.count_ones() as usize);

                if options_count == 0 {
                    return None; // No options available, backtrack immediately
                }

                if options_count < fewest_options {
                    best_cell = Some((i, j));
                    fewest_options = options_count;

                    if fewest_options == 1 {
                        break 'outer;
                    }
                }   
            }
            if fewest_options == 1 {
                break 'outer;
            }
        }
    }

    if let Some((i, j)) = best_cell {
        let unavailable = get_unavailable(row_sets, col_sets, square_sets, i, j);

        for n in 1..=9 {
            if unavailable & (1 << n) == 0 {
                sudoku[i][j] = n as u8;
                row_sets[i] |= 1 << n;
                col_sets[j] |= 1 << n;
                square_sets[i / 3][j / 3] |= 1 << n;

                if let Some(solution) = backtrack(sudoku, row_sets, col_sets, square_sets) {
                    return Some(solution);
                }

                sudoku[i][j] = 0;
                row_sets[i] &= !(1 << n);
                col_sets[j] &= !(1 << n);
                square_sets[i / 3][j / 3] &= !(1 << n);
            }
        }

        None
    } else {
        // No empty cells, Sudoku is solved
        Some(*sudoku)
    }
}