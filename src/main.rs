use std::{io::BufRead, path::PathBuf};

use wordle_solver::{
    dictionary::Dictionary,
    puzzle::{Puzzle, PuzzleError},
    solver::NaiveSolver,
};

#[derive(Debug)]
enum Error {
    Args(&'static str),
    Io(std::io::Error),
    Puzzle(PuzzleError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<PuzzleError> for Error {
    fn from(e: PuzzleError) -> Self {
        Error::Puzzle(e)
    }
}

fn main() -> Result<(), Error> {
    let dict_path: PathBuf = std::env::args()
        .skip(1)
        .next()
        .ok_or(Error::Args("Provide dictionary path as first argument"))?
        .into();
    let file = std::fs::File::open(dict_path)?;
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, std::io::Error>>()?;

    let dict = Dictionary::new(lines);
    let puzzle = Puzzle::new(5, &dict)?;
    let mut solver = NaiveSolver::new();
    let res = solver.solve(&puzzle, &dict);
    println!("Solved: {:?}", res);
    Ok(())
}
