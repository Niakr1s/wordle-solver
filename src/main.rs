use std::io::BufRead;

use args::Command;
use clap::Parser;
use wordle_solver::{dictionary::Dictionary, puzzle::Puzzle, solver::NaiveSolver};

fn main() -> Result<(), error::Error> {
    let args = args::Args::parse();
    let file = std::fs::File::open(args.dict_path)?;
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, std::io::Error>>()?;
    let dict = Dictionary::new(lines);

    run(&dict, args.length, args.subcommands)
}

fn run(dict: &Dictionary, word_length: usize, cmd: Command) -> Result<(), error::Error> {
    match cmd {
        Command::Bench { tries } => {
            let mut solver = NaiveSolver::new();
            let start = std::time::Instant::now();
            for _ in 0..tries {
                solver.solve(&Puzzle::new(word_length, dict)?, dict)?;
            }
            let duration = start.elapsed().as_millis();
            println!(
                "NaiveSolver: {}ms per each try",
                duration as f64 / tries as f64
            );
        }
    }
    Ok(())
}

mod args {
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        /// Path to dictionary which is a simple file with one word per line.
        #[arg(short, long)]
        pub dict_path: PathBuf,

        /// Length of the word
        #[arg(short, long, default_value = "5")]
        pub length: usize,

        #[command(subcommand)]
        pub subcommands: Command,
    }

    #[derive(Debug, Subcommand)]
    pub enum Command {
        /// Benchmark
        Bench {
            #[arg(short, long, default_value = "10")]
            tries: u32,
        },
    }
}

mod error {
    use wordle_solver::{puzzle::PuzzleError, solver::SolveError};

    #[derive(Debug)]
    pub enum Error {
        Io(std::io::Error),
        Puzzle(PuzzleError),
        Solve(SolveError),
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

    impl From<SolveError> for Error {
        fn from(e: SolveError) -> Self {
            Error::Solve(e)
        }
    }
}
