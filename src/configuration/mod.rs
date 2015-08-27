//! The configuration that the password generator uses to generate a password

pub mod defaults;

/// The base configuration struct for the password generator. Pass this configuration to the
/// generator to create a password.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Configuration {
    pub words: WordConfiguration,
    pub seperator: SeperatorConfiguration,
    pub padding_digits: PaddingDigitConfiguration,
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
    /// // Print out the default configuration
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
    /// // Print out the default configuration
    /// println!("{:?}", config);
    /// ```
    pub fn ntml() -> Configuration {
        return defaults::configuration_ntml();
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct WordConfiguration {
    pub num_words: u8,
    pub min_length: u8,
    pub max_length: u8,

    pub transformations: WordTransformations,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum WordTransformations {
    CapitaliseFirst,
    CapitaliseNonFirst,
    LowerCase,
    UpperCase,
    AlternatingLowerUpper,
    RandomLowerUpper,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct SeperatorConfiguration {
    pub seperator_type: SeperatorTypes,
    pub seperators: Vec<char>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum SeperatorTypes {
    SingleCharacter,
    RandomCharacter,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PaddingDigitConfiguration {
    pub num_before: u8,
    pub num_after: u8,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PaddingSymbolConfiguration {
    pub padding_type: PaddingTypes,
    pub padding_character_type: PaddingCharTypes,
    pub padding_chars: Vec<char>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum PaddingTypes {
    // Pad to the length given
    Adaptive(u32),
    // Padding before and padding after
    Fixed(u8, u8),
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum PaddingCharTypes {
    SingleCharacter,
    RandomCharacter,
    SeperatorCharacter,
}
