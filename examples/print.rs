//! This example prints out a generated XKCD-style password and exits

extern crate xkcd_pass;

use xkcd_pass::{SimpleEnglish, Configuration, generate_password};

pub fn main() {
    let word_list = SimpleEnglish::new();
    let config = Configuration::default();
    let password = generate_password(&config, &word_list);
   
    println!("Password: {}", password);
}