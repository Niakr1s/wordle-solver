use rand::{seq::IteratorRandom, thread_rng};
use std::ops::{Deref, DerefMut};

use crate::dictionary::Dictionary;

#[derive(Debug)]
pub enum PuzzleError {
    NoWordsForLen(usize),
}

#[derive(Debug)]
pub enum CheckWordError {
    InvalidLength,
    WordNotGuessed,
}

pub struct WordCheck(Vec<(char, CharCheck)>);

impl WordCheck {
    pub fn word(&self) -> String {
        self.0.iter().map(|(c, _)| c).collect()
    }

    pub fn is_solved(&self) -> bool {
        self.iter()
            .all(|(_, ty)| matches!(ty, CharCheck::GuessedExact))
    }
}

impl Deref for WordCheck {
    type Target = Vec<(char, CharCheck)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WordCheck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<(char, CharCheck)>> for WordCheck {
    fn from(v: Vec<(char, CharCheck)>) -> Self {
        WordCheck(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum CharCheck {
    GuessedExact,
    GuessedWrongPlace,
    NotGuessed,
}

pub struct Puzzle {
    word: String,
}

impl Puzzle {
    pub fn new(length: usize, dic: &Dictionary) -> Result<Puzzle, PuzzleError> {
        let words = dic.get(length).ok_or(PuzzleError::NoWordsForLen(length))?;
        let mut rng = thread_rng();
        let word = words
            .get_words()
            .iter()
            .choose(&mut rng)
            .ok_or(PuzzleError::NoWordsForLen(length))?
            .clone();
        Ok(Puzzle { word })
    }

    pub fn check_word(&self, word: &str) -> Result<WordCheck, CheckWordError> {
        let word = word.to_lowercase();
        if word.chars().count() != self.word.chars().count() {
            return Err(CheckWordError::InvalidLength);
        }

        let mut word_check: WordCheck = vec![].into();
        for (cw, w) in self.word.chars().zip(word.chars()) {
            let char_check = if cw == w {
                CharCheck::GuessedExact
            } else {
                if self.word.contains(w) {
                    CharCheck::GuessedWrongPlace
                } else {
                    CharCheck::NotGuessed
                }
            };

            word_check.push((w, char_check));
        }
        Ok(word_check)
    }

    pub fn len(&self) -> usize {
        self.word.chars().count()
    }
}

#[cfg(test)]
mod test_puzzle {
    use super::*;

    const WORDS: &[&str; 4] = &["def", "asd", "abc", "xyz"];

    fn make_dic(words: &[&str]) -> Dictionary {
        Dictionary::new(words.into_iter().map(|&s| s.to_owned()).collect())
    }

    #[test]
    fn test_check_word_lowercases_input_word() {
        let puzzle = Puzzle {
            word: "abc".to_owned(),
        };

        let w = "ABC";
        let check = puzzle.check_word(w).unwrap();
        assert_eq!(check.word(), w.to_lowercase());
        assert_eq!(check.len(), 3);
        for (check_ch, _) in check.iter().zip(w.chars()) {
            assert_eq!(check_ch.1, CharCheck::GuessedExact);
        }
    }

    #[test]
    fn test_check_word_exact() {
        let puzzle = Puzzle {
            word: "abc".to_owned(),
        };

        let w = "abc";
        let check = puzzle.check_word(w).unwrap();
        assert_eq!(check.word(), w);
        assert_eq!(check.len(), 3);
        for (check_ch, w_ch) in check.iter().zip(w.chars()) {
            assert_eq!(check_ch.0, w_ch);
            assert_eq!(check_ch.1, CharCheck::GuessedExact);
        }
    }

    #[test]
    fn test_check_word_wrong() {
        let puzzle = Puzzle {
            word: "abc".to_owned(),
        };

        let w = "def";
        let check = puzzle.check_word(w).unwrap();
        assert_eq!(check.word(), w);
        assert_eq!(check.len(), 3);
        for (check_ch, w_ch) in check.iter().zip(w.chars()) {
            assert_eq!(check_ch.0, w_ch);
            assert_eq!(check_ch.1, CharCheck::NotGuessed);
        }
    }

    #[test]
    fn test_check_word_wrong_place() {
        let puzzle = Puzzle {
            word: "abc".to_owned(),
        };

        let w = "cab";
        let check = puzzle.check_word(w).unwrap();
        assert_eq!(check.word(), w);
        assert_eq!(check.len(), 3);
        for (check_ch, w_ch) in check.iter().zip(w.chars()) {
            assert_eq!(check_ch.0, w_ch);
            assert_eq!(check_ch.1, CharCheck::GuessedWrongPlace);
        }
    }

    #[test]
    fn test_check_word_1() {
        let puzzle = Puzzle {
            word: "abc".to_owned(),
        };

        let w = "bad";
        let check = puzzle.check_word(w).unwrap();
        assert_eq!(check.word(), w);
        assert_eq!(check.len(), 3);
        assert_eq!(check[0].1, CharCheck::GuessedWrongPlace);
        assert_eq!(check[1].1, CharCheck::GuessedWrongPlace);
        assert_eq!(check[2].1, CharCheck::NotGuessed);
    }

    #[test]
    fn test_new_is_random() {
        let dic = make_dic(WORDS);

        for _ in 0.. {
            let puzzle1 = Puzzle::new(3, &dic).unwrap();
            let puzzle2 = Puzzle::new(3, &dic).unwrap();
            if puzzle1.word != puzzle2.word {
                return;
            }
        }
        unreachable!()
    }
}
