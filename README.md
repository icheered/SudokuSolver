# Rust Sudoku Solver
Just trying a few different algorithms to try and solve it as fast as possible.

## Usage
The `time` argument is optional and will run the solver 10 times and print the average time and standard deviation it took to solve the puzzle. Run the following command in the `src` folder.
```bash
cargo run --quiet --release time
```
### Scoreboard
Pure backtracking: 591 ± 9.91 ms