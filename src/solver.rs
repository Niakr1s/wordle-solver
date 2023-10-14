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

pub struct NaiveSolver;

impl NaiveSolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve(&mut self, puzzle: &Puzzle, dict: &Dictionary) -> Result<String, SolveError> {
        let len = puzzle.len();
        let words = dict.get(len).ok_or(SolveError::EmptyDictionary)?;
        let mut rng = thread_rng();

        let mut remained_words: HashSet<&String> = words.get_words().iter().collect();

        let mut chooser = WordsChooser::new();

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
                    CharCheck::GuessedExact => chooser.add_wanted_at_pos(i, *ch),
                    CharCheck::GuessedWrongPlace => chooser.add_wanted_not_at_pos(i, *ch),
                    CharCheck::NotGuessed => chooser.add_except(*ch),
                }
            }
            remained_words = chooser.filter(remained_words);
        }
        Err(SolveError::SolutionNotFound)
    }
}

struct WordsChooser {
    /// usize = position
    wanted_at_pos: HashMap<usize, char>,

    /// usize = position not wanted
    wanted_not_at_pos: HashMap<usize, char>,

    except: HashSet<char>,
}

impl WordsChooser {
    pub fn new() -> Self {
        Self {
            wanted_at_pos: HashMap::new(),
            wanted_not_at_pos: HashMap::new(),
            // wanted: HashSet::new(),
            except: HashSet::new(),
        }
    }

    pub fn add_wanted_at_pos(&mut self, pos: usize, ch: char) {
        self.wanted_at_pos.insert(pos, ch);
        self.except.remove(&ch);
    }

    pub fn add_wanted_not_at_pos(&mut self, pos: usize, ch: char) {
        self.wanted_not_at_pos.insert(pos, ch);
        self.except.remove(&ch);
    }

    pub fn add_except(&mut self, ch: char) {
        let keys_to_remove: Vec<_> = self
            .wanted_at_pos
            .iter()
            .filter_map(|(&k, &v)| if v == ch { Some(k) } else { None })
            .collect();
        for k in keys_to_remove {
            self.wanted_at_pos.remove(&k);
        }
        self.except.insert(ch);
    }

    pub fn filter<'a>(&self, words: HashSet<&'a String>) -> HashSet<&'a String> {
        words
            .iter()
            .filter(|x| {
                let chars: Vec<_> = x.chars().collect();
                for (&i, &ch) in &self.wanted_at_pos {
                    if chars[i] != ch {
                        return false;
                    }
                }
                for (&i, &ch) in &self.wanted_not_at_pos {
                    if chars[i] == ch {
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
