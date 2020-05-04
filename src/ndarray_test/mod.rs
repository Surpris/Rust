//! ndarray_test
//!
//! provide test functions using ndarray crate

// #[macro_use(s)]
// extern crate ndarray;
use ndarray::{arr1, arr2, Array};

/// test to call arr1 and arr2
pub fn test_ndarray_call() -> bool {
    let vec1 = arr1(&[1, 2, 3]);
    let mat1 = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(vec1.sum(), 6);
    assert_eq!(mat1.shape(), [3, 3]);
    true
}

/// test to call Array
pub fn test_ndarray_array() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr = Array::from(vec1);
    let mat1 = arr.into_shape((3, 3)).unwrap();
    assert_eq!(mat1, arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
    true
}

/// test to use a slice
pub fn test_ndarray_slice() -> bool {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mat1 = Array::from(vec1).into_shape((3, 3)).unwrap();
    assert_eq!(mat1.slice(s![0..2, 1..3]), arr2(&[[2, 3], [5, 6]]));
    true
}