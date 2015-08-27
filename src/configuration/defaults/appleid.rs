//! The Apple ID configuration for the password generator.

use super::super::*;

pub fn configuration_appleid() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 3,
            min_length: 5,
            max_length: 7,

            transformations: WordTransformations::RandomLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::RandomCharacter,
            seperators: vec!['-', ':', '.', ','],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 2,
            num_after: 2,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(1, 1),
            padding_character_type: PaddingCharTypes::RandomCharacter,
            padding_chars: vec!['!', '?', '@', '&'],
        },
    };

}
