//! # Rust Examples
//!
//! This library provides some functions created in studying Rust.

// To use macros in a crate, #[macro_use] must be called in 
// the root file of the crate.
#[macro_use(s, array)]
extern crate ndarray;
extern crate rustfft;
extern crate cast;

pub mod ndarray_test;
pub mod rustfft_test;