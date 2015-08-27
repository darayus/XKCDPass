//! A set of common password generators with preset configurations.
//!
//! The functions that generate configurations all follow the naming convention
//! `configuration_<name>`.
//!
//! # Example
//!
//! The following code generates the default configuration
//!
//! ```
//! use xkcd_pass::configuration::defaults::default::configuration_default;
//!
//! let config = configuration_default();
//! // Print out the default configuration
//! println!("{:?}", config);
//! ```

pub mod default;
pub mod appleid;
pub mod ntml;
pub mod xkcd;

pub use self::default::configuration_default;
pub use self::appleid::configuration_appleid;
pub use self::ntml::configuration_ntml;
pub use self::xkcd::configuration_xkcd;
