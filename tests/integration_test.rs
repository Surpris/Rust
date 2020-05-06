//! ndarray_tests.rs
//!
//! test functions in the ndarray_test module

extern crate rust_examples;
use rust_examples as rex;

#[test]
fn test_ndarray() {
    rex::ndarray_test::test_ndarray_call();
    rex::ndarray_test::test_ndarray_array();
    rex::ndarray_test::test_ndarray_slice();
    rex::ndarray_test::test_ndarray_transpose();
    rex::ndarray_test::test_ndarray_ops();
    rex::ndarray_test::test_ndarray_permute();
    rex::ndarray_test::test_ndarray_functions();
}

#[test]
fn test_rustfft() {
    rex::rustfft_test::test_fft_vec();
    rex::rustfft_test::test_fft_sin(1024);
    rex::rustfft_test::test_fft_non_concurrency(1024, 4);
    rex::rustfft_test::test_fft_concurrency(1024, 4);
}