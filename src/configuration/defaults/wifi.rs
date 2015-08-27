//! The wifi configuration for the password generator.

use super::super::*;

pub fn configuration_wifi() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 6,
            min_length: 4,
            max_length: 8,

            transformations: WordTransformations::RandomLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['-', '+', '=', '.', '*', '_', '|', '~', ','],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 4,
            num_after: 4,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Adaptive(63),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '@', '$', '%', '^', '&', '*', '+', '=', ':', '|', '~', '?'],
        },
    };

}
