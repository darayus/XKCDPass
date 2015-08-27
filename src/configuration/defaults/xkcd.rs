//! The XKCD configuration for the password generator.

use super::super::*;

pub fn configuration_xkcd() -> Configuration {

    return Configuration {
        words: WordConfiguration {
            num_words: 4,
            min_length: 4,
            max_length: 8,

            transformations: WordTransformations::RandomLowerUpper,
        },
        seperator: SeperatorConfiguration {
            seperator_type: SeperatorTypes::SingleCharacter,
            seperators: vec!['-'],
        },
        padding_digits: PaddingDigitConfiguration {
            num_before: 0,
            num_after: 0,
        },
        padding_symbols: PaddingSymbolConfiguration {
            padding_type: PaddingTypes::Fixed(0, 0),
            padding_character_type: PaddingCharTypes::SingleCharacter,
            padding_chars: vec!['-'],
        },
    };

}
