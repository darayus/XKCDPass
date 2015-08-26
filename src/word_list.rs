
// TODO: Break this into structs with a WordList trait that allows custom dictionaries

/// Generate the word list. Currently only loads the simple english dictionary. This is loaded
/// at compile time. Returns a list of words.
pub fn get_word_list(min_len: u8, max_len: u8) -> Vec<String> {
    let dict_simple_en = include_str!("dictionaries/simple_en.txt");

    // TODO: More efficient parsing of the dictionaries
    let words: Vec<&str> = dict_simple_en.split('\n').collect();
    let words: Vec<String> = words.iter().filter(|&a| a.len() >= min_len as usize && a.len() <= max_len as usize)
                                  .filter(|&a| a.trim().len() > 0).map(|a| a.to_string()).collect();

    return words;
}
