# Rust Sudoku Solver
Just trying a few different algorithms to try and solve it as fast as possible.
For testing I'm using [3 million sudoku puzzles](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings/data) from Kaggle. These are all solved within 2 minutes with an average duration of 37 ± 65.86 ms.

## Usage
Download the [3 million sudoku puzzles](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings/data) from Kaggle and place the `input.csv` file in the root of the project (I would include it in the repo but it is more than 500 MB). Then run the following command to start solving the puzzles.
```bash
cargo run --quiet --release
```