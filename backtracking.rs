pub fn sudoku_is_valid(sudoku: &Vec<Vec<u8>>) -> bool {
    // Check if there are no repeated numbers in the rows
    for line in sudoku {
        let mut numbers = line.clone();
        // Sort and remove 0's
        numbers.sort();
        numbers.retain(|&c| c != 0);
        for i in 0..numbers.len()-1 {
            if numbers[i] == numbers[i+1] {
                return false
            }
        }
    }

    // Check if there are no repeated numbers in the columns
    for i in 0..sudoku.len() {
        let mut numbers = sudoku.iter().map(|line| line[i]).collect::<Vec<u8>>();
        
        // Sort and remove 0's
        numbers.sort();
        numbers.retain(|&c| c != 0);
        for j in 0..numbers.len()-1 {
            if numbers[j] == numbers[j+1] {
                return false
            }
        }
    }

    // Check if there are no repeated numbers in the 3x3 squares
    for i in 0..sudoku.len()/3 {
        for j in 0..sudoku.len()/3 {
            let mut numbers = Vec::new();
            for k in 0..3 {
                for l in 0..3 {
                    numbers.push(sudoku[3*i+k][3*j+l]);
                }
            }
            // Sort and remove 0's
            numbers.sort();
            numbers.retain(|&c| c != 0);
            for m in 0..numbers.len()-1 {
                if numbers[m] == numbers[m+1] {
                    return false
                }
            }
        }
    }
    true
}


// Solve the sudoku using backtracking. Return the solved sudoku.
pub fn backtrack(sudoku: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut sudoku = sudoku.clone();

    for i in 0..sudoku.len() {
        for j in 0..sudoku.len() {
            if sudoku[i][j] != 0 {
                continue; // Skip filled cells
            }
            // Try to fill the empty cell with a number
            for n in 1..10 {
                sudoku[i][j] = n;
                if sudoku_is_valid(&sudoku) {
                    // If the sudoku is still valid, continue with the next cell
                    let solution = backtrack(&sudoku);
                    if solution.len() > 0 {
                        return solution
                    }
                }
            }
            // If no number works, return an empty vector
            return Vec::new()
        }
    }

    return sudoku
}
