//! The configuration that the password generator uses to generate a password
//!
//! For most passwords, you will just want to use one of the preset password configurations.
//! These presets can be found in the defaults module but convenience methods have also been
//! provided.
//!
//! # Examples
//!
//! ```
//! use xkcd_pass::Configuration;
//!
//! // Make a new default configuration
//! let default_config = Configuration::default();
//! // Make a new xkcd configuration
//! let xkcd_config = Configuration::xkcd();
//! ```

pub mod defaults;

/// The base configuration struct for the password generator. Pass this configuration to the
/// generator to create a password.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// The configuration for the quantity and style of words generated
    pub words: WordConfiguration,
    /// The configuration for the seperator characters between words
    pub seperator: SeperatorConfiguration,
    /// The configuration for the padding digits before and after the password
    pub padding_digits: PaddingDigitConfiguration,
    /// The configuration for the padding symbols before and after the password
    pub padding_symbols: PaddingSymbolConfiguration,
}

impl Configuration {
    /// Creates a new configuration with the default settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::default();
    /// // Print out the default configuration
    /// println!("{:?}", config);
    /// ```
    pub fn default() -> Configuration {
        return defaults::configuration_default();
    }

    /// Creates a new configuration with the Apple ID settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::appleid();
    /// // Print out the appleid configuration
    /// println!("{:?}", config);
    /// ```
    pub fn appleid() -> Configuration {
        return defaults::configuration_appleid();
    }

    /// Creates a new configuration with the NTML settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::ntml();
    /// // Print out the ntml configuration
    /// println!("{:?}", config);
    /// ```
    pub fn ntml() -> Configuration {
        return defaults::configuration_ntml();
    }

    /// Creates a new configuration with the XKCD settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::xkcd();
    /// // Print out the xkcd configuration
    /// println!("{:?}", config);
    /// ```
    pub fn xkcd() -> Configuration {
        return defaults::configuration_xkcd();
    }

    /// Creates a new configuration with the web16 settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::web16();
    /// // Print out the web16 configuration
    /// println!("{:?}", config);
    /// ```
    pub fn web16() -> Configuration {
        return defaults::configuration_web16();
    }

    /// Creates a new configuration with the web32 settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::web32();
    /// // Print out the web32 configuration
    /// println!("{:?}", config);
    /// ```
    pub fn web32() -> Configuration {
        return defaults::configuration_web32();
    }

    /// Creates a new configuration with the wifi settings loaded in.
    ///
    /// ```
    /// use xkcd_pass::configuration::Configuration;
    ///
    /// let config = Configuration::wifi();
    /// // Print out the wifi configuration
    /// println!("{:?}", config);
    /// ```
    pub fn wifi() -> Configuration {
        return defaults::configuration_wifi();
    }
}

/// The configuration for the quantity and style of words generated
#[derive(Debug, Serialize, Deserialize)]
pub struct WordConfiguration {
    /// The number of words that should be in the password
    pub num_words: u8,
    /// The minimum length of the words in the password
    pub min_length: u8,
    /// The maximum length of the words in the password
    pub max_length: u8,

    /// The transformations that should be applied to the words
    pub transformations: WordTransformations,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WordTransformations {
    /// Capitalise the first letter of every word. i.e `Random` and `Word`
    CapitaliseFirst,
    /// Capitalise all but the first letter of every word. i.e `rANDOM` and `wORD`
    CapitaliseNonFirst,
    /// Convert all the words into lowercase. i.e `lower` and `case`
    LowerCase,
    /// Convert all the words into uppercase. i.e `UPPER` and `CASE`
    UpperCase,
    /// Alternate the words between lower and upper case in the password. i.e `randomWORDSinPASSWORD`
    AlternatingLowerUpper,
    /// Randomly convert each word in the password to uppercase or lowercase. i.e `ArandomWORDPASSWORD`
    RandomLowerUpper,
}

/// The configuration for the seperator characters between words
#[derive(Debug, Serialize, Deserialize)]
pub struct SeperatorConfiguration {
    /// The method in which the seperator chosen during password generation
    pub seperator_type: SeperatorTypes,
    /// The possible seperator characters that could be chosen. Must contain at least 1 character
    pub seperators: Vec<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SeperatorTypes {
    /// Use the same character (the first character in the seperators vector) for the seperator
    /// between all of the words
    SingleCharacter,
    /// Use a random character for the seperator between each word
    RandomCharacter,
}

/// The configuration for the padding digits before and after the password
#[derive(Debug, Serialize, Deserialize)]
pub struct PaddingDigitConfiguration {
    /// The number of digits to pad before the password
    pub num_before: u8,
    /// The number of digits to pad after the password
    pub num_after: u8,
}

/// The configuration for the padding symbols before and after the password
#[derive(Debug, Serialize, Deserialize)]
pub struct PaddingSymbolConfiguration {
    /// The padding style to use for the password
    pub padding_type: PaddingTypes,
    /// The method in which the padding characters are chosen
    pub padding_character_type: PaddingCharTypes,
    /// The choices of characters that can pad the password. Must contain at least 1 character
    pub padding_chars: Vec<char>,
}

/// The padding style to use for the password
#[derive(Debug, Serialize, Deserialize)]
pub enum PaddingTypes {
    /// Pad the password to the length given. If the password is only 10 characters long and this
    /// is set to 16, 6 extra characters of padding will be added to the end of the password
    Adaptive(u32),
    /// A fixed amount of padding before and padding after the password. The first value is the
    /// amount of padding before the password and the second value is the amount of padding
    /// after the password
    Fixed(u8, u8),
}

/// The method in which the padding characters are chosen
#[derive(Debug, Serialize, Deserialize)]
pub enum PaddingCharTypes {
    /// Only a single character will be used for the padding. The first character in the
    /// `padding_chars` vector in `PaddingSymbolConfiguration` is used
    SingleCharacter,
    /// Choose a random character to pad the password from the `padding_chars` vector in
    /// `PaddingSymbolConfiguration`
    RandomCharacter,
    /// Use the same character as the one chosen for the seperator character
    SeperatorCharacter,
}
