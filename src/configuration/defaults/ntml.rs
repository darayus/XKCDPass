//! The NTML configuration for the password generator.

use super::super::*;

pub fn configuration_ntml() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 2,
            min_length: 5,
            max_length: 5,

            transformations: WordTransformations::CapitaliseNonFirst,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['-', '+', '=', '.', '*', '_', '|', '~', ','],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 1,
            num_after: 0,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(0, 1),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '@', '$', '%', '^', '&', '*', '+', '=', ':', '|', '~', '?'],
        },
    };

}
