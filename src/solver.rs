use std::collections::{HashMap, HashSet};

use rand::{seq::IteratorRandom, thread_rng};

use crate::{
    dictionary::Dictionary,
    puzzle::{CharCheck, CheckWordError, Puzzle, WordCheck},
};

#[derive(Debug)]
pub enum SolveError {
    EmptyDictionary,
    SolutionNotFound,
    CheckWordError(CheckWordError),
}

impl From<CheckWordError> for SolveError {
    fn from(e: CheckWordError) -> SolveError {
        SolveError::CheckWordError(e)
    }
}

pub struct NaiveSolver {
    chooser: WordsChooser,
}

impl NaiveSolver {
    pub fn new() -> Self {
        Self {
            chooser: WordsChooser::new(),
        }
    }

    pub fn solve(&mut self, puzzle: &Puzzle, dict: &Dictionary) -> Result<String, SolveError> {
        let len = puzzle.len();
        let words = dict.get_words(len).ok_or(SolveError::EmptyDictionary)?;
        let mut rng = thread_rng();

        let mut remained_words: HashSet<&String> = words.iter().collect();

        for i in 1.. {
            println!("NaiveSolver: iteration #{i}");

            let word = remained_words
                .iter()
                .choose(&mut rng)
                .ok_or(SolveError::SolutionNotFound)?;

            let check: WordCheck = puzzle.check_word(&word)?;
            if check.is_solved() {
                return Ok(check.word());
            }
            for (i, (ch, ty)) in (&check).iter().enumerate() {
                match ty {
                    CharCheck::GuessedExact => self.chooser.add_exact(i, *ch),
                    CharCheck::GuessedWrongPlace => self.chooser.add_wanted(*ch),
                    CharCheck::NotGuessed => {}
                }
            }
            remained_words = self.chooser.choose(remained_words);
        }
        Err(SolveError::SolutionNotFound)
    }
}

struct WordsChooser {
    /// usize = position
    exact: HashMap<usize, char>,
    wanted: HashSet<char>,
    except: HashSet<char>,
}

impl WordsChooser {
    pub fn new() -> Self {
        Self {
            exact: HashMap::new(),
            wanted: HashSet::new(),
            except: HashSet::new(),
        }
    }

    pub fn add_exact(&mut self, pos: usize, ch: char) {
        self.exact.insert(pos, ch);
        self.except.remove(&ch);
    }

    pub fn add_wanted(&mut self, ch: char) {
        self.wanted.insert(ch);
        self.except.remove(&ch);
    }

    pub fn add_except(&mut self, ch: char) {
        let keys_to_remove: Vec<_> = self
            .exact
            .iter()
            .filter_map(|(&k, &v)| if v == ch { Some(k) } else { None })
            .collect();
        for k in keys_to_remove {
            self.exact.remove(&k);
        }
        self.wanted.remove(&ch);
        self.except.insert(ch);
    }

    pub fn choose<'a>(&self, words: HashSet<&'a String>) -> HashSet<&'a String> {
        words
            .iter()
            .filter(|x| {
                let chars: Vec<_> = x.chars().collect();
                for (&i, &ch) in &self.exact {
                    if chars[i] != ch {
                        return false;
                    }
                }
                for wanted in &self.wanted {
                    if !chars.contains(&wanted) {
                        return false;
                    }
                }
                for except in &self.except {
                    if chars.contains(&except) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }
}
