use indicatif::{ProgressBar, ProgressStyle};
use color_eyre::Report;
use std::fs;
use std::time::Instant;

mod solver;
mod utility;


fn main() -> Result<(), Report> {
    color_eyre::install()?;

    // Check if the flag --time is present
    let args: Vec<String> = std::env::args().collect();
    let time = args.len() > 1 && args[1] == "time";

    // Read the content of the file
    let input = fs::read_to_string("input.txt")?;
    let sudoku = utility::parse_input(&input);
    utility::print_sudoku(&sudoku, &sudoku);

    let repeat_times = if time { 10 } else { 1 };
    let mut times = vec![0; repeat_times];
    
    let result = solver::solve(&sudoku);

    // Set up the progress bar
    let progress_bar = ProgressBar::new(repeat_times as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")?
        .progress_chars("#>-"));

    for i in 0..repeat_times {
        let start = Instant::now();
        let _ = solver::solve(&sudoku);
        let duration = start.elapsed();
        times[i] = duration.as_millis();

        // Update the progress bar
        progress_bar.inc(1);
    }

    // Finish the progress bar
    progress_bar.finish_with_message("Completed");

    utility::print_sudoku(&sudoku, &result);

    // Print the solution
    let std = utility::standard_deviation(&times);
    println!("Average time taken to solve the sudoku: {} ± {:.2} ms", times.iter().sum::<u128>() / repeat_times as u128, std);

    Ok(())
}