//! ndarray_tests.rs
//! 
//! test functions in the ndarray_test module

extern crate rust_examples;
use rust_examples as rex;

#[test]
fn test_ndarray_call() {
    assert!(rex::ndarray_test::test_ndarray_call());
}

#[test]
fn test_ndarray_array() {
    assert!(rex::ndarray_test::test_ndarray_array());
}

#[test]
fn test_ndarray_slice() {
    assert!(rex::ndarray_test::test_ndarray_slice());
}

#[test]
fn test_ndarray_reversed_axes() {
    assert!(rex::ndarray_test::test_ndarray_transpose());
}

#[test]
fn test_ndarray_ops() {
    assert!(rex::ndarray_test::test_ndarray_ops());
}

#[test]
fn test_ndarray_permute() {
    assert!(rex::ndarray_test::test_ndarray_permute());
}