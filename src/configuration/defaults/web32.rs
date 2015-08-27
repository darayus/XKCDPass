//! The web32 configuration for the password generator.

use super::super::*;

pub fn configuration_web32() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 4,
            min_length: 4,
            max_length: 5,

            transformations: WordTransformations::AlternatingLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['-', '+', '=', '.', '*', '_', '|', '~', ','],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 2,
            num_after: 2,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(1, 1),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '@', '$', '%', '^', '&', '*', '+', '=', ':', '|', '~', '?'],
        },
    };

}
