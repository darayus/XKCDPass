//! Contains the word lists for the password generator.
//!
//! Custom word lists can be made by creating a struct / enum and implementing the `WordList`
//! trait.

pub mod simple_english;
#[cfg(test)]
mod tests;

pub use self::simple_english::SimpleEnglish;

/// The trait that all word lists must implement.
pub trait WordList {
    /// Get all the words in the word list which are within the minimum and maximum length
    fn get_words(&self, min_len: u8, max_len: u8) -> Vec<&String>;
}
