XKCD Password Generator
=======================

[![Build Status](https://travis-ci.org/darayus/XKCDPass.svg?branch=master)](https://travis-ci.org/darayus/XKCDPass)

A password generation library based on https://xkpasswd.net/ perl module for the Rust programming library. Documentation can be found [here](https://docs.darayus.com/XKCDPass/xkcd_pass/).

```rust
use xkcd_pass::{SimpleEnglish, Configuration, generate_password};

// Use the provided English dictionary
let word_list = SimpleEnglish::new();
// Use the default configuration. Custom configs can be used
let config = Configuration::default();
// Generate the password
let password = generate_password(&config, &word_list);

println!("Password: {}", password);
// Password: ||18+make+MERCURY+present+99||
```

## Configuration presets

Currently all the presets from https://xkpasswd.net/ are available in this library:

*  Apple ID
*  NTML
*  Web 16
*  Web 32
*  Wifi
*  XKCD

## Custom configurations

The default configuration is shown below:

```rust
Configuration {
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
}
```

The description of the fields can be found in the documentation.
