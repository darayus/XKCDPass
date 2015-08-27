//! Generates XKCD-style passwords
//!
//! # Example
//!
//! ```
//! use xkcd_pass::{SimpleEnglish, Configuration, generate_password};
//!
//! let word_list = SimpleEnglish::new();
//! let config = Configuration::default();
//! let password = generate_password(&config, &word_list);
//!
//! println!("Password: {}", password);
//! ```

extern crate rand;
extern crate rustc_serialize;

pub mod generator;
pub mod configuration;
pub mod word_list;

pub use generator::generate_password;
pub use configuration::Configuration;
pub use word_list::SimpleEnglish;
