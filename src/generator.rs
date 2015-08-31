
use std::ascii::AsciiExt;
use std::iter::repeat;
use rand::{Rng, thread_rng};
use super::configuration::{Configuration, WordTransformations, SeperatorTypes, PaddingCharTypes, PaddingTypes};
use super::word_list::WordList;

/// Generates a password with the given configuration and word list.
///
/// ```
/// use xkcd_pass::{generate_password, Configuration, SimpleEnglish};
///
/// let config = Configuration::default();
/// let word_list = SimpleEnglish::new();
/// // Generate 1 password.
/// let password = generate_password(&config, &word_list);
/// println!("Password: {}", password);
/// ```
pub fn generate_password<A: WordList>(config: &Configuration, word_list: &A) -> String {
    // TODO: Accept different random generators
    let mut rgen = thread_rng();

    // Generate the words
    let word_list = word_list.get_words(config.words.min_length, config.words.max_length);
    let num_words = config.words.num_words as usize;
    let mut chosen_words = Vec::with_capacity(num_words);

    for _ in (0..num_words) {
        let random_index = rgen.gen_range(0, word_list.len());
        chosen_words.push(word_list[random_index].clone());
    }
    // Lowercase all the words
    for i in (0..chosen_words.len()) {
        chosen_words[i] = to_lower_case(&chosen_words[i]);
    }

    // Modify the case of the words
    match config.words.transformations {
        WordTransformations::CapitaliseFirst => {
            for i in (0..chosen_words.len()) {
                let final_word: String = {
                    let word = &chosen_words[i];
                    let mut word_iter = word.chars();
                    let mut first_char = word_iter.next().unwrap();
                    first_char = first_char.to_ascii_uppercase();
                    let rest_of_word: String = word_iter.collect();
                    let mut final_str = first_char.to_string();
                    final_str.push_str(&rest_of_word);
                    final_str
                };
                chosen_words[i] = final_word;
            }
        },
        WordTransformations::CapitaliseNonFirst => {
            for i in (0..chosen_words.len()) {
                let final_word: String = {
                    let word = &chosen_words[i];
                    let mut word_iter = word.chars();
                    let mut first_char = word_iter.next().unwrap();
                    first_char = first_char.to_ascii_lowercase();
                    let rest_of_word: String = word_iter.map(|a| a.to_ascii_uppercase()).collect();
                    let mut final_str = first_char.to_string();
                    final_str.push_str(&rest_of_word);
                    final_str
                };
                chosen_words[i] = final_word;
            }
        },
        WordTransformations::LowerCase => {
            // Do nothing since all the words are already lower case
        },
        WordTransformations::UpperCase => {
            for i in (0..chosen_words.len()) {
                chosen_words[i] = to_upper_case(&chosen_words[i]);
            }
        },
        WordTransformations::AlternatingLowerUpper => {
            for i in (0..chosen_words.len()) {
                if i % 2 == 1 {
                    // Make every second word upper case
                    chosen_words[i] = to_upper_case(&chosen_words[i]);
                }
            }
        },
        WordTransformations::RandomLowerUpper => {
            for i in (0..chosen_words.len()) {
                if rgen.gen() {
                    // Make every second word upper case
                    chosen_words[i] = to_upper_case(&chosen_words[i]);
                }
            }
        },
    }

    // Determine the seperator character
    let num_seperators = config.seperator.seperators.len();
    assert!(num_seperators > 0);
    let sep_char = match config.seperator.seperator_type {
        SeperatorTypes::SingleCharacter => {
            config.seperator.seperators[0]
        }
        SeperatorTypes::RandomCharacter => {
            let random_index = rgen.gen_range(0, num_seperators);
            config.seperator.seperators[random_index]
        }
    };

    // Determine the padding character
    let num_pad_chars = config.padding_symbols.padding_chars.len();
    let pad_char = match config.padding_symbols.padding_character_type {
        PaddingCharTypes::SingleCharacter => {
            assert!(num_pad_chars > 0);
            config.padding_symbols.padding_chars[0]
        },
        PaddingCharTypes::RandomCharacter => {
            assert!(num_pad_chars > 0);
            let random_index = rgen.gen_range(0, num_pad_chars);
            config.padding_symbols.padding_chars[random_index]
        },
        PaddingCharTypes::SeperatorCharacter => {
            sep_char
        },
    };

    // Determine the digits before and after
    let num_digits_before = config.padding_digits.num_before;
    let num_digits_after = config.padding_digits.num_after;
    let mut digits_before_str = String::new();
    let mut digits_after_str = String::new();
    if num_digits_before > 0 {
        let digits_before = rgen.gen_range(10u64.pow((num_digits_before - 1) as u32), 10u64.pow(num_digits_before as u32));
        digits_before_str = digits_before.to_string();
    }
    if num_digits_after > 0 {
        let digits_after = rgen.gen_range(10u64.pow((num_digits_after - 1) as u32), 10u64.pow(num_digits_after as u32));
        digits_after_str = digits_after.to_string();
    }

    // Build the final password

    // Generate the string without the padding
    let mut pass = String::from("");
    if num_digits_before > 0 {
        let temp_dig = format!("{}{}", digits_before_str, sep_char);
        pass.push_str(&temp_dig);
    }

    for (i, word) in chosen_words.iter().enumerate() {
        if i != 0 {
            pass.push_str(&sep_char.to_string());
        }
        pass.push_str(&word);
    }

    if num_digits_after > 0 {
        let temp_dig = format!("{}{}", sep_char, digits_after_str);
        pass.push_str(&temp_dig);
    }

    // Generate the padding
    match config.padding_symbols.padding_type {
        PaddingTypes::Adaptive(min_length) => {
            let pass_length = pass.len();
            let min_length = min_length as usize; // TODO: Use usize in config ?
            if min_length > pass_length {
                let needed_padding = min_length - pass_length;
                let end_pad: String = repeat(pad_char).take(needed_padding).collect();

                pass = format!("{}{}", pass, end_pad);
            }
        },
        PaddingTypes::Fixed(num_pad_before, num_pad_after) => {
            let begin_pad: String = repeat(pad_char).take(num_pad_before as usize).collect();
            let end_pad: String = repeat(pad_char).take(num_pad_after as usize).collect();

            pass = format!("{}{}{}", begin_pad, pass, end_pad);
        },
    }

    return pass;
}

fn to_lower_case(s: &str) -> String {
    return s.to_ascii_lowercase();
}

fn to_upper_case(s: &str) -> String {
    return s.to_ascii_uppercase();
}
