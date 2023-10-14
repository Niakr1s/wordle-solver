use std::collections::{HashMap, HashSet};

/// Inner struct, used by [Dictionary].
pub struct Words {
    words: HashSet<String>,
    freqs: HashMap<char, usize>,
}

impl Words {
    fn new(words: HashSet<String>) -> Words {
        let freqs = words
            .iter()
            .flat_map(|w| w.chars())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_default() += 1;
                acc
            });
        Words { words, freqs }
    }

    pub fn get_words(&self) -> &HashSet<String> {
        &self.words
    }

    pub fn get_freqs(&self) -> &HashMap<char, usize> {
        &self.freqs
    }
}

pub struct Dictionary {
    words_by_len: HashMap<usize, Words>,
}

pub struct DictionaryBuilder;

impl Dictionary {
    pub fn new(words: Vec<String>) -> Dictionary {
        let words_by_len: HashMap<usize, Words> = words
            .into_iter()
            .map(|s| {
                let w = s.trim().to_lowercase().to_owned();
                (w.chars().count(), w)
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<usize, HashSet<String>>, (length, word)| {
                    acc.entry(length).or_default().insert(word);
                    acc
                },
            )
            .into_iter()
            .map(|(length, words)| (length, Words::new(words)))
            .collect();

        Dictionary { words_by_len }
    }

    pub fn get(&self, length: usize) -> Option<&Words> {
        self.words_by_len.get(&length)
    }
}

#[cfg(test)]
mod test_dictionary_builder {
    use super::*;

    fn make_dic(words: &[&str]) -> Dictionary {
        Dictionary::new(words.into_iter().map(|&s| s.to_owned()).collect())
    }

    #[test]
    fn test_from_vec_works() {
        let words = &["a", "b", "bc", "de", "def", "asd"];
        let dic = make_dic(words);
        assert_eq!(dic.words_by_len.len(), 3);
        assert!(dic.get(1).unwrap().get_words().contains("a"));
        assert!(dic.get(1).unwrap().get_words().contains("b"));
        assert!(dic.get(2).unwrap().get_words().contains("bc"));
        assert!(dic.get(2).unwrap().get_words().contains("de"));
        assert!(dic.get(3).unwrap().get_words().contains("def"));
        assert!(dic.get(3).unwrap().get_words().contains("asd"));
    }

    #[test]
    fn test_from_vec_works_and_trims_whitespaces() {
        let dic = make_dic(&["abc", "de ", " a ", "\na\n", "\r\n def \r\n"]);
        assert_eq!(dic.words_by_len.len(), 3);
        assert!(dic.get(1).unwrap().get_words().contains("a"));
        assert!(dic.get(2).unwrap().get_words().contains("de"));
        assert!(dic.get(3).unwrap().get_words().contains("abc"));
        assert!(dic.get(3).unwrap().get_words().contains("def"));
    }

    #[test]
    fn test_from_vec_works_and_lowercases_words() {
        let dic = make_dic(&["ABC", "DEf"]);
        assert_eq!(dic.words_by_len.len(), 1);
        assert_eq!(dic.words_by_len.get(&3).unwrap().get_words().len(), 2);
        assert!(dic.get(3).unwrap().get_words().contains("abc"));
        assert!(dic.get(3).unwrap().get_words().contains("def"));
    }
}
