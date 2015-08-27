use super::{WordList, SimpleEnglish};

#[test]
pub fn test_dictionary_contains_words() {
    let dict = SimpleEnglish::new();
    let words = dict.get_words(0, 5);

    assert!(words.len() != 0);
    // Make sure all the words are less than 5 characters long
    for word in words {
        assert!(word.len() <= 5);
    }
}
