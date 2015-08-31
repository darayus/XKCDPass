XKCD Password Generator
=======================

[![Build Status](https://travis-ci.org/darayus/XKCDPass.svg?branch=master)](https://travis-ci.org/darayus/XKCDPass)

A password generation library based on thexkpasswd.pm perl module for the Rust programming library. 

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
