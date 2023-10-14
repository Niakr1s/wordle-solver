use std::{io::BufRead, path::PathBuf};

use wordle_solver::{dictionary::Dictionary, puzzle::Puzzle, solver::NaiveSolver};

fn main() {
    let dict_path: PathBuf = std::env::args().skip(1).next().unwrap().into();
    let file = std::fs::File::open(dict_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    let dict = Dictionary::new(lines);
    let puzzle = Puzzle::new(5, &dict).unwrap();
    let mut solver = NaiveSolver::new();
    let res = solver.solve(&puzzle, &dict);
    println!("Solved: {:?}", res);
}
