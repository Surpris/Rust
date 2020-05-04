//! integration_test.rs
//! 
//! test all the functions in the library

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