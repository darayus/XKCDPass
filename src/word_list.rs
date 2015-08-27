//! Contains the word lists for the password generator.
//!
//! Custom word lists can be made by creating a struct / enum and implementing the `WordList`
//! trait.


/// The trait that all word lists must implement.
pub trait WordList {
    /// Get all the words in the word list which are within the minimum and maximum length
    fn get_words(&self, min_len: u8, max_len: u8) -> Vec<&String>;
}

/// Generate the word list. Currently only loads the simple english dictionary. This is loaded
/// at compile time. Returns a list of words.
pub struct SimpleEnglish(Vec<String>);

impl SimpleEnglish {
    /// Creates a new `SimpleEnglish` word list that can be used to generate passwords.
    ///
    /// ```
    /// let word_list = SimpleEnglish::new();
    /// // The word list has words!
    /// assert!(word_list.num_words() != 0);
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
