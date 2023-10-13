use std::path::PathBuf;

use wordle_solver::{dictionary::Dictionary, puzzle::Puzzle, solver::NaiveSolver};

fn main() {
    let dict_path: PathBuf = std::env::args().skip(1).next().unwrap().into();
    let dict = Dictionary::new().from_file(&dict_path).unwrap();
    let puzzle = Puzzle::new(5, &dict).unwrap();
    let mut solver = NaiveSolver::new();
    let res = solver.solve(&puzzle, &dict);
    println!("Solved: {:?}", res);
}
