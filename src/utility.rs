use std::f64;
use crate::types::{Grid, GRID_SIZE};

pub fn print_sudoku(original_sudoku: & Grid, sudoku: & Grid) {
    // Replace zero for space, add a vertical line every 3 characters
    // Add a horizontal line every 3 lines
    // If the character in the solution is also in the original sudoku, print it in bold green
    for (i, line) in sudoku.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            println!("---------------------");
        }
        print!(" ");
        for (j, c) in line.iter().enumerate() {
            if j > 0 && j % 3 == 0 {
                print!("|");
            }
            if *c == 0 {
                print!("  ");
            } else if original_sudoku[i][j] == *c {
                print!("\x1b[1;32m{}\x1b[0m ", c);
            } else {
                print!("{} ", c);
            }
            if j == 8 {
                println!();
            }
        }
    }
    println!();
}

pub fn parse_input(input: &str) -> Grid {
    let mut sudoku = [[0u8; GRID_SIZE]; GRID_SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            sudoku[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }
    sudoku
}

pub fn standard_deviation(data: &[u128]) -> f64 {
    let count = data.len() as f64;
    if count == 0.0 {
        return 0.0;
    }

    let mean = data.iter().sum::<u128>() as f64 / count;
    let variance = data.iter().map(|value| {
        let diff = *value as f64 - mean;
        diff * diff
    }).sum::<f64>() / count;

    variance.sqrt()
}
