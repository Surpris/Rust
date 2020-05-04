//! integration_test.rs
//! 
//! test all the functions in the library

extern crate rust_examples;
use rust_examples as rex;

#[test]
fn test_ndarray_call() {
    assert!(rex::ndarray::test_ndarray_call());
}

#[test]
fn test_ndarray_Array() {
    assert!(rex::ndarray::test_ndarray_Array());
}