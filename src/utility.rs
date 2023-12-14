
use std::f64;
use crate::types::{Grid};

use csv;
use serde_derive::Deserialize;
use std::io::{self, ErrorKind};


#[derive(Deserialize)]
struct RawRecord {
    id: i64,
    puzzle: String,
    solution: String,
    clues: i8,
    difficulty: f64,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: i64,
    pub puzzle: Grid,
    pub solution: Grid,
    pub clues: i8,
    pub difficulty: f64,
}

use std::fmt;

#[derive(Debug)]
pub struct CustomError(Box<dyn std::error::Error + Send + Sync>);
impl From<csv::Error> for CustomError {
    fn from(err: csv::Error) -> CustomError {
        CustomError(Box::new(err))
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}

impl From<Box<dyn std::error::Error + Send + Sync>> for CustomError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> CustomError {
        CustomError(err)
    }
}

fn string_to_array(input: &str) -> Result<[[u8; 9]; 9], CustomError> {
    let mut array = [[0u8; 9]; 9];
    let chars: Vec<char> = input.chars().collect();

    if chars.len() != 81 {
        return Err(CustomError(Box::new(io::Error::new(ErrorKind::InvalidData, "Invalid input length"))));
    }

    for (i, &c) in chars.iter().enumerate() {
        array[i / 9][i % 9] = match c.to_digit(10) {
            Some(digit) => digit as u8,
            None => return Err(CustomError(Box::new(io::Error::new(ErrorKind::InvalidData, "Invalid character in input")))),
        };
    }

    Ok(array)
}


pub fn read_csv(file_path: &str, number: usize) -> Result<Vec<Record>, CustomError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut records = Vec::new();

    for (index, result) in rdr.deserialize::<RawRecord>().enumerate() {
        let raw_record: RawRecord = result?;
        let puzzle = string_to_array(&raw_record.puzzle.replace('.', "0"))?;
        let solution = string_to_array(&raw_record.solution)?;

        records.push(Record {
            id: raw_record.id,
            puzzle,
            solution,
            clues: raw_record.clues,
            difficulty: raw_record.difficulty,
        });

        if index >= number-1 && number != 0 { 
            break;
        }
    }

    Ok(records)
}

#[allow(dead_code)]
pub fn print_sudoku(original_sudoku: &Grid, solved_sudoku: &Grid, is_correct: bool, solve_time: u128) {
    println!("|    Original Puzzle       |         Solved Puzzle        |                 Result                 |");
    println!("■--------------------------■------------------------------■----------------------------------------■");

    for i in 0..9 {
        if i % 3 == 0 && i > 0 {
            println!("-------------------------  ■  --------------------------  ■");
        }

        for j in 0..9 {
            // Print original puzzle
            if j % 3 == 0 && j > 0 { print!(" | "); }
            print!("{:2}", if original_sudoku[i][j] == 0 { ' ' } else { char::from_digit(original_sudoku[i][j] as u32, 10).unwrap() });

            // Space between puzzles
            if j == 8 { print!("   ■   "); }
        }

        for j in 0..9 {
            // Print solved puzzle
            if j % 3 == 0 && j > 0 { print!(" | "); }
            if solved_sudoku[i][j] == 0 {
                print!("  ");
            } else if original_sudoku[i][j] == solved_sudoku[i][j] {
                print!("\x1b[1;32m{:2}\x1b[0m", solved_sudoku[i][j]);
            } else {
                print!("{:2}", solved_sudoku[i][j]);
            }
        }

        // Print result on the end of each line
        if i == 4 {
            println!("   ■ Solution is {} (Time: {} ms)", if is_correct { "correct" } else { "incorrect" }, solve_time);
        } else {
            println!("   ■");
        }
    }

    println!();
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
