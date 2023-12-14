use indicatif::{ProgressBar, ProgressStyle};
use color_eyre::{eyre::Report, eyre::Result};
use std::time::Instant;

mod types;
mod solver;
mod utility;
mod reduced_options;


fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let records = utility::read_csv("input.csv", 0)?;
    let repeat_times = records.len();

    // Set up the progress bar
    let progress_bar = ProgressBar::new(repeat_times as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")?
        .progress_chars("#>-"));

    let mut times = Vec::with_capacity(repeat_times);

    // Format repeat_times so that it is human readable (space every 3 digits)
    let repeat_times_hr = repeat_times.to_string().chars().rev().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<String>>().join(" ").chars().rev().collect::<String>();
    println!("Solving {} sudokus...", repeat_times_hr);
    // Loop through all the puzzles
    for record in records {
        let start = Instant::now();
        let result_option = solver::solve(&record.puzzle); // Assuming `solve` function returns an Option
        let duration = start.elapsed();
    
        let solve_time = duration.as_micros();
        times.push(solve_time);
        
        match result_option {
            Some(result) => {
                let correct = result == record.solution;
                if !correct {
                    println!("Incorrect solution");
                } else {
                    //utility::print_sudoku(&record.puzzle, &result, correct, solve_time);
                }
                
            },
            None => {
                println!("Failed to solve puzzle");
            }
        }
    
        // Update the progress bar
        progress_bar.inc(1);
    }

    // Finish the progress bar
    progress_bar.finish_with_message("Completed");

    // Print total time
    println!("Total time taken to solve all the sudokus: {} ms", times.iter().sum::<u128>() as f64 / 1000.0);

    // Calculate and print statistics
    let avg_time = times.iter().sum::<u128>() / repeat_times as u128;
    let std_dev = utility::standard_deviation(&times); // Assuming utility::standard_deviation exists
    println!("Average time taken to solve the sudoku: {} ± {:.2} ms", avg_time, std_dev);

    Ok(())
}
