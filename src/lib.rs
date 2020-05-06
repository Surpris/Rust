//! # Rust Examples
//!
//! This library provides some functions created in studying Rust.

// To use macros in a crate, #[macro_use] must be called in
// the root file of the crate.

extern crate cast;
// extern crate fftw;
#[macro_use(s, array)]
extern crate ndarray;
extern crate num_complex;
// extern crate num_cpus;
extern crate num_traits;
extern crate rustfft;

pub mod example_utils;
// pub mod fftw_test;
pub mod ndarray_test;
pub mod rustfft_test;
pub mod thread_test;