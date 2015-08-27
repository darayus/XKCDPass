//! The simple English word list based on the `simple_en.txt` dictionary.

use super::WordList;

/// The English word list. Currently only loads the simple english dictionary. This is loaded
/// at compile time. Returns a list of words.
pub struct SimpleEnglish(Vec<String>);

impl SimpleEnglish {
    /// Creates a new `SimpleEnglish` word list that can be used to generate passwords.
    ///
    /// ```
    /// use xkcd_pass::word_list::SimpleEnglish;
    ///
    /// let word_list = SimpleEnglish::new();
    /// ```
    pub fn new() -> SimpleEnglish {
        let dict_simple_en = include_str!("dictionaries/simple_en.txt");

        // TODO: More efficient parsing of the dictionaries
        let words: Vec<&str> = dict_simple_en.split('\n').collect();
        let words: Vec<String> = words.iter()
                                      .filter(|&a| a.trim().len() > 0).map(|a| a.to_string()).collect();

        return SimpleEnglish(words);
    }
}

impl WordList for SimpleEnglish {
    fn get_words(&self, min_len: u8, max_len: u8) -> Vec<&String> {
        let &SimpleEnglish(ref words) = self;
        return words.iter().filter(|&a| a.len() >= min_len as usize && a.len() <= max_len as usize).collect();
    }
}
