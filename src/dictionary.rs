use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    path::Path,
};

pub type Words = HashSet<String>;

pub struct Dictionary {
    words: HashMap<usize, Words>,
}

pub struct DictionaryBuilder;

impl Dictionary {
    pub fn new() -> DictionaryBuilder {
        DictionaryBuilder {}
    }

    pub fn get_words(&self, length: usize) -> Option<&Words> {
        self.words.get(&length)
    }
}

#[derive(Debug)]
pub enum DictionaryBuildError {
    Io(std::io::Error),
}

pub type DictionaryBuildResult = std::result::Result<Dictionary, DictionaryBuildError>;

impl From<std::io::Error> for DictionaryBuildError {
    fn from(e: std::io::Error) -> DictionaryBuildError {
        DictionaryBuildError::Io(e)
    }
}

impl DictionaryBuilder {
    pub fn from_file(&self, path: &Path) -> DictionaryBuildResult {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let lines: std::result::Result<Vec<_>, std::io::Error> = reader.lines().collect();
        self.from_vec(lines?)
    }

    pub fn from_vec(&self, words: Vec<String>) -> DictionaryBuildResult {
        let words: HashMap<usize, Words> = words
            .into_iter()
            .map(|s| {
                let w = s.trim().to_lowercase().to_owned();
                (w.chars().count(), w)
            })
            .fold(HashMap::new(), |mut acc, (length, word)| {
                acc.entry(length).or_default().insert(word);
                acc
            });
        Ok(Dictionary { words })
    }
}

#[cfg(test)]
mod test_dictionary_builder {
    use super::*;

    fn make_dic(words: &[&str]) -> DictionaryBuildResult {
        let builder = DictionaryBuilder;
        builder.from_vec(words.into_iter().map(|&s| s.to_owned()).collect())
    }

    #[test]
    fn test_from_vec_works() {
        let words = &["a", "b", "bc", "de", "def", "asd"];
        let dic = make_dic(words).unwrap();
        assert_eq!(dic.words.len(), 3);
        assert!(dic.get_words(1).unwrap().contains("a"));
        assert!(dic.get_words(1).unwrap().contains("b"));
        assert!(dic.get_words(2).unwrap().contains("bc"));
        assert!(dic.get_words(2).unwrap().contains("de"));
        assert!(dic.get_words(3).unwrap().contains("def"));
        assert!(dic.get_words(3).unwrap().contains("asd"));
    }

    #[test]
    fn test_from_vec_works_and_trims_whitespaces() {
        let dic = make_dic(&["abc", "de ", " a ", "\na\n", "\r\n def \r\n"]).unwrap();
        assert_eq!(dic.words.len(), 3);
        assert!(dic.get_words(1).unwrap().contains("a"));
        assert!(dic.get_words(2).unwrap().contains("de"));
        assert!(dic.get_words(3).unwrap().contains("abc"));
        assert!(dic.get_words(3).unwrap().contains("def"));
    }

    #[test]
    fn test_from_vec_works_and_lowercases_words() {
        let dic = make_dic(&["ABC", "DEf"]).unwrap();
        assert_eq!(dic.words.len(), 1);
        assert_eq!(dic.words.get(&3).unwrap().len(), 2);
        assert!(dic.get_words(3).unwrap().contains("abc"));
        assert!(dic.get_words(3).unwrap().contains("def"));
    }
}
