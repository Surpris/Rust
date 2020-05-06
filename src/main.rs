//! # Rust Examples
//!
//! This library provides some functions created in studying Rust.

// To use macros in a crate, #[macro_use] must be called in 
// the root file of the crate.

extern crate rust_examples;
use rust_examples as rex;

pub fn main() {
    println!("Hello, world!");
}