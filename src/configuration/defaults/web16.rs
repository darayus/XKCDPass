//! The web16 configuration for the password generator.

use super::super::*;

pub fn configuration_web16() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 3,
            min_length: 4,
            max_length: 4,

            transformations: WordTransformations::RandomLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['-', '+', '=', '.', '*', '_', '|', '~', ','],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 0,
            num_after: 0,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(1, 1),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '@', '$', '%', '^', '&', '*', '+', '=', ':', '|', '~', '?'],
        },
    };

}
