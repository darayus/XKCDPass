//! The default configuration for the password generator.

use super::super::*;

pub fn configuration_default() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 3,
            min_length: 4,
            max_length: 8,

            transformations: WordTransformations::AlternatingLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['!', '@', '$', '%', '^', '&', '*', '-', '_', '+', '=', ':', '|', '~', '?', '/', '.', ';'],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 2,
            num_after: 2,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(2, 2),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '@', '$', '%', '^', '&', '*', '-', '_', '+', '=', ':', '|', '~', '?', '/', '.', ';'],
        },
    };

}
